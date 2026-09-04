//! Calldata and storage mutation decoder.

use alloy::{
    primitives::{Bytes, FixedBytes, U256},
    providers::Provider,
    rpc::types::{BlockId, BlockNumberOrTag, TransactionRequest},
    sol_types::SolCall,
};
use anyhow::{Context, Result};
use rsg_types::StorageOp;

use crate::{
    abi::IRocketStorageRead,
    constants::{PRE_BLOCK, ROCKET_STORAGE},
    parser::{
        parse_address_from_args, parse_address_result, parse_bytes_from_args, parse_bytes_result,
        parse_string_from_args, parse_string_result, parse_u256_result,
    },
};

/// Decode a mutator call made to RocketStorage:
/// extracts the raw key, queries the pre-upgrade value at `PRE_BLOCK`, and decodes the new value.
pub async fn decode_mutator_call<P: Provider>(
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
                .call(
                    TransactionRequest::default()
                        .to(ROCKET_STORAGE)
                        .input(calldata.into()),
                )
                .block(block_id)
                .await
                .unwrap_or_default()
        }};
    }

    let b32_key: FixedBytes<32> = key_bytes.into();

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
            let old_val = raw
                .last()
                .map(|&b| (b != 0).to_string())
                .unwrap_or_else(|| "false".into());
            let new_val = if op.is_delete() {
                "false".into()
            } else {
                args.get(63)
                    .map(|&b| (b != 0).to_string())
                    .unwrap_or_else(|| "false".into())
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
