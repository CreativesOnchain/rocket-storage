//! Live trace capture engine.
//!
//! Uses `debug_traceTransaction` (via alloy 2.x) to extract all RocketStorage
//! mutation calls from the Saturn 1 upgrade transaction.
//!
//! Also reads pre-call values using typed getter calls at the pre-upgrade block,
//! producing a complete `FrozenTrace` for offline attestation.

use alloy::{
    primitives::{address, Address, Bytes, B256, U256},
    providers::{ext::DebugApi, Provider, ProviderBuilder},
    rpc::types::{
        trace::geth::{
            CallConfig, CallFrame, GethDebugBuiltInTracerType, GethDebugTracingOptions, GethTrace,
        },
        BlockId, BlockNumberOrTag, TransactionRequest,
    },
    sol,
    sol_types::SolCall,
};
use anyhow::{Context, Result};
use rsg_decode::KeyCatalogue;
use rsg_types::{
    FrozenTrace, ObservedEffect, ObservedExternalCall, PinnedFixture, StorageOp,
};
use std::str::FromStr;

// ─── RocketStorage read interface ─────────────────────────────────────────────

sol! {
    #[allow(missing_docs)]
    interface IRocketStorageRead {
        function getAddress(bytes32 _key) external view returns (address);
        function getUint(bytes32 _key) external view returns (uint256);
        function getBool(bytes32 _key) external view returns (bool);
        function getBytes32(bytes32 _key) external view returns (bytes32);
        function getInt(bytes32 _key) external view returns (int256);
        function getString(bytes32 _key) external view returns (string memory);
        function getBytes(bytes32 _key) external view returns (bytes memory);
    }
}

// ─── Constants ────────────────────────────────────────────────────────────────

const ROCKET_STORAGE: Address =
    address!("1d8f8f00cfa6758d7be78336684788fb0ee0fa46");
const UPGRADE_TX: &str =
    "0x2fc10aad3c1b00bdfa9b6fddab79e0f2688609848f8f7a1a6449ab42da38530c";
const PRE_BLOCK: u64 = 24_479_993;
const EXEC_BLOCK: u64 = 24_479_994;
const SOURCE_COMMIT: &str = "fb7d9c428dc3dddc3fbd3e634e3cb365655df89e";
const UPGRADE_CONTRACT: Address =
    address!("5b3b5c76391662e56d0ff72f31b89c409316c8ba");

// ─── Public API ──────────────────────────────────────────────────────────────

/// Capture the Saturn 1 upgrade trace from a live archive RPC.
pub async fn capture_live(rpc_url: &str) -> Result<FrozenTrace> {
    let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);

    // Validate chain ID
    let chain_id = provider
        .get_chain_id()
        .await
        .context("failed to get chain ID")?;
    if chain_id != 1 {
        anyhow::bail!("expected Ethereum mainnet (chain_id=1), got {chain_id}");
    }

    // Validate and fetch pre-upgrade block
    let pre_block_info = provider
        .get_block(BlockId::Number(BlockNumberOrTag::Number(PRE_BLOCK)))
        .await
        .context("failed to fetch pre-upgrade block")?
        .context("pre-upgrade block not found — is this an archive node?")?;

    let pre_block_hash = format!("{:?}", pre_block_info.header.hash);
    eprintln!("[rsg] Pre-block {PRE_BLOCK} hash: {pre_block_hash}");
    eprintln!("[rsg] Calling debug_traceTransaction…");

    // Call debug_traceTransaction with CallTracer
    let tx_hash = B256::from_str(UPGRADE_TX).context("invalid tx hash")?;
    let trace_opts = GethDebugTracingOptions::new_tracer(GethDebugBuiltInTracerType::CallTracer)
        .with_call_config(CallConfig {
            only_top_call: Some(false),
            with_log: Some(false),
        });

    let trace = provider
        .debug_trace_transaction(tx_hash, trace_opts)
        .await
        .context("debug_traceTransaction failed")?;

    let root_frame = match trace {
        GethTrace::CallTracer(frame) => frame,
        _ => anyhow::bail!("unexpected trace type: expected CallTracer"),
    };

    eprintln!("[rsg] Trace received. Walking call tree…");

    let catalogue = KeyCatalogue::build();
    let mut effects: Vec<ObservedEffect> = Vec::new();
    let mut external_calls: Vec<ObservedExternalCall> = Vec::new();
    let mut call_index = 0usize;

    walk_calls(
        &root_frame,
        &provider,
        &catalogue,
        &mut effects,
        &mut external_calls,
        &mut call_index,
    )
    .await?;

    eprintln!(
        "[rsg] Capture complete: {} mutations, {} external calls",
        effects.len(),
        external_calls.len()
    );

    let pinned = PinnedFixture {
        chain_id,
        pre_block: PRE_BLOCK,
        pre_block_hash,
        upgrade_tx: UPGRADE_TX.to_string(),
        exec_block: EXEC_BLOCK,
        upgrade_contract: format!("{UPGRADE_CONTRACT:?}"),
        rocket_storage: format!("{ROCKET_STORAGE:?}"),
        source_commit: SOURCE_COMMIT.to_string(),
        replay_tool: format!("rsg-capture/{}", env!("CARGO_PKG_VERSION")),
    };

    Ok(FrozenTrace {
        pinned,
        effects,
        external_calls,
    })
}

// ─── Call tree walker ─────────────────────────────────────────────────────────

async fn walk_calls<P>(
    frame: &CallFrame,
    provider: &P,
    catalogue: &KeyCatalogue,
    effects: &mut Vec<ObservedEffect>,
    external_calls: &mut Vec<ObservedExternalCall>,
    call_index: &mut usize,
) -> Result<()>
where
    P: Provider,
{
    let to_addr = frame.to.unwrap_or(Address::ZERO);
    let from_addr = frame.from;
    let input = frame.input.as_ref();

    if input.len() >= 4 && to_addr == ROCKET_STORAGE {
        let sel = hex::encode(&input[0..4]);
        if let Some(op) = StorageOp::from_selector(&sel) {
            match decode_mutator_call(provider, &op, input).await {
                Ok((raw_key, old_val, new_val)) => {
                    let semantic = catalogue.lookup_typed_hex(&raw_key, &op).map(|s| s.to_string());
                    effects.push(ObservedEffect {
                        call_index: *call_index,
                        caller: format!("{from_addr:?}"),
                        op,
                        raw_key,
                        semantic_path: semantic,
                        old_value: old_val,
                        new_value: new_val,
                    });
                }
                Err(e) => {
                    eprintln!("[rsg] WARN decode[{}]: {e}", call_index);
                }
            }
        }
    } else if to_addr != ROCKET_STORAGE
        && to_addr != Address::ZERO
        && input.len() >= 4
        && from_addr == UPGRADE_CONTRACT
    {
        let selector = format!("0x{}", hex::encode(&input[0..4]));
        let eth_value = frame.value.map(|v| v.to_string()).unwrap_or_else(|| "0".into());
        external_calls.push(ObservedExternalCall {
            call_index: *call_index,
            from: format!("{from_addr:?}"),
            to: format!("{to_addr:?}"),
            selector,
            eth_value,
            success: frame.error.is_none(),
        });
    }

    *call_index += 1;

    for child in &frame.calls {
        Box::pin(walk_calls(
            child,
            provider,
            catalogue,
            effects,
            external_calls,
            call_index,
        ))
        .await?;
    }

    Ok(())
}

// ─── Mutator call decoder ─────────────────────────────────────────────────────

async fn decode_mutator_call<P: Provider>(
    provider: &P,
    op: &StorageOp,
    input: &[u8],
) -> Result<(String, String, String)> {
    let args = input.get(4..).context("calldata too short")?;
    let key_bytes: [u8; 32] = args
        .get(..32)
        .context("no key bytes")?
        .try_into()
        .unwrap();
    let raw_key = format!("0x{}", hex::encode(key_bytes));

    let block_id = BlockId::Number(BlockNumberOrTag::Number(PRE_BLOCK));

    // Helper: call a view function on RocketStorage at the pre-upgrade block
    macro_rules! eth_call {
        ($call:expr) => {{
            let calldata = Bytes::from($call.abi_encode());
            provider
                .call(TransactionRequest::default()
                    .to(ROCKET_STORAGE)
                    .input(calldata.into()))
                .block(block_id)
                .await
                .unwrap_or_default()
        }};
    }

    let b32_key: alloy::primitives::FixedBytes<32> = key_bytes.into();

    let (old, new) = match op {
        StorageOp::SetAddress | StorageOp::DeleteAddress => {
            let raw = eth_call!(IRocketStorageRead::getAddressCall { _key: b32_key });
            let old_addr = parse_address_result(&raw);
            let new_addr = if op.is_delete() {
                "0x0000000000000000000000000000000000000000".into()
            } else {
                parse_address_from_args(args, 32)
            };
            (old_addr, new_addr)
        }
        StorageOp::SetUint | StorageOp::AddUint | StorageOp::SubUint | StorageOp::DeleteUint => {
            let raw = eth_call!(IRocketStorageRead::getUintCall { _key: b32_key });
            let old_val = parse_u256_result(&raw);
            let new_val = if op.is_delete() {
                "0".into()
            } else {
                args.get(32..64)
                    .map(|b| U256::from_be_slice(b).to_string())
                    .unwrap_or_else(|| "0".into())
            };
            (old_val, new_val)
        }
        StorageOp::SetBool | StorageOp::DeleteBool => {
            let raw = eth_call!(IRocketStorageRead::getBoolCall { _key: b32_key });
            let old_val = raw.last().map(|&b| (b != 0).to_string()).unwrap_or_else(|| "false".into());
            let new_val = if op.is_delete() {
                "false".into()
            } else {
                args.get(63).map(|&b| (b != 0).to_string()).unwrap_or_else(|| "false".into())
            };
            (old_val, new_val)
        }
        StorageOp::SetBytes32 => {
            let raw = eth_call!(IRocketStorageRead::getBytes32Call { _key: b32_key });
            let old_val = format!("0x{}", hex::encode(&raw[..32.min(raw.len())]));
            let new_val = args
                .get(32..64)
                .map(|b| format!("0x{}", hex::encode(b)))
                .unwrap_or_else(|| format!("0x{}", "0".repeat(64)));
            (old_val, new_val)
        }
        StorageOp::SetString | StorageOp::DeleteString => {
            let raw = eth_call!(IRocketStorageRead::getStringCall { _key: b32_key });
            let old_val = parse_string_result(&raw);
            let new_val = if op.is_delete() {
                String::new()
            } else {
                parse_string_from_args(args, 32)
            };
            (old_val, new_val)
        }
        StorageOp::SetBytes | StorageOp::DeleteBytes => {
            let raw = eth_call!(IRocketStorageRead::getBytesCall { _key: b32_key });
            let old_val = parse_bytes_result(&raw);
            let new_val = if op.is_delete() {
                "0x".into()
            } else {
                parse_bytes_from_args(args, 32)
            };
            (old_val, new_val)
        }
        _ => ("unknown".into(), "unknown".into()),
    };

    Ok((raw_key, old, new))
}

fn parse_address_result(raw: &[u8]) -> String {
    if raw.len() >= 32 {
        format!("0x{}", hex::encode(&raw[12..32]))
    } else {
        "0x0000000000000000000000000000000000000000".into()
    }
}

fn parse_address_from_args(args: &[u8], offset: usize) -> String {
    if args.len() >= offset + 20 {
        format!("0x{}", hex::encode(&args[offset + 12..offset + 32]))
    } else {
        "0x0000000000000000000000000000000000000000".into()
    }
}

fn parse_u256_result(raw: &[u8]) -> String {
    if raw.len() >= 32 {
        U256::from_be_slice(&raw[..32]).to_string()
    } else {
        "0".into()
    }
}

fn parse_string_from_args(args: &[u8], offset_in_args: usize) -> String {
    if args.len() >= offset_in_args + 32 {
        let offset = U256::from_be_slice(&args[offset_in_args..offset_in_args + 32]).to::<usize>();
        if args.len() >= offset + 32 {
            let len = U256::from_be_slice(&args[offset..offset + 32]).to::<usize>();
            let start = offset + 32;
            let end = start + len;
            if args.len() >= end {
                return String::from_utf8_lossy(&args[start..end]).to_string();
            }
        }
    }
    String::new()
}

fn parse_string_result(raw: &[u8]) -> String {
    if raw.len() >= 64 {
        let len = U256::from_be_slice(&raw[32..64]).to::<usize>();
        let start = 64;
        let end = 64 + len;
        if raw.len() >= end {
            return String::from_utf8_lossy(&raw[start..end]).to_string();
        }
    }
    String::new()
}

fn parse_bytes_from_args(args: &[u8], offset_in_args: usize) -> String {
    if args.len() >= offset_in_args + 32 {
        let offset = U256::from_be_slice(&args[offset_in_args..offset_in_args + 32]).to::<usize>();
        if args.len() >= offset + 32 {
            let len = U256::from_be_slice(&args[offset..offset + 32]).to::<usize>();
            let start = offset + 32;
            let end = start + len;
            if args.len() >= end {
                return format!("0x{}", hex::encode(&args[start..end]));
            }
        }
    }
    "0x".to_string()
}

fn parse_bytes_result(raw: &[u8]) -> String {
    if raw.len() >= 64 {
        let len = U256::from_be_slice(&raw[32..64]).to::<usize>();
        let start = 64;
        let end = 64 + len;
        if raw.len() >= end {
            return format!("0x{}", hex::encode(&raw[start..end]));
        }
    }
    "0x".to_string()
}

// ─── Fixture I/O ─────────────────────────────────────────────────────────────

/// Load a previously frozen trace from a JSON file (offline mode).
pub fn load_frozen_trace(path: &std::path::Path) -> Result<FrozenTrace> {
    let data = std::fs::read_to_string(path)
        .with_context(|| format!("cannot read fixture: {}", path.display()))?;
    serde_json::from_str(&data)
        .with_context(|| format!("cannot parse fixture: {}", path.display()))
}

/// Save a captured trace to a JSON file.
pub fn save_frozen_trace(trace: &FrozenTrace, path: &std::path::Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(trace)?;
    std::fs::write(path, json)?;
    eprintln!("[rsg] Frozen trace saved to {}", path.display());
    Ok(())
}
