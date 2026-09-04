//! Live trace capture engine.
//!
//! Uses `debug_traceTransaction` (via alloy) to extract all RocketStorage
//! mutation calls from the Saturn 1 upgrade transaction.
//!
//! Also reads pre-call values using typed getter calls at the pre-upgrade block,
//! producing a complete `FrozenTrace` for offline attestation.

pub mod abi;
pub mod constants;
pub mod decoder;
pub mod io;
pub mod parser;
pub mod walker;

pub use constants::*;
pub use io::{load_frozen_trace, save_frozen_trace};

use std::str::FromStr;

use alloy::{
    primitives::B256,
    providers::{ext::DebugApi, Provider, ProviderBuilder},
    rpc::types::{
        trace::geth::{CallConfig, GethDebugBuiltInTracerType, GethDebugTracingOptions, GethTrace},
        BlockId, BlockNumberOrTag,
    },
};
use anyhow::{Context, Result};
use rsg_decode::KeyCatalogue;
use rsg_types::{FrozenTrace, ObservedEffect, ObservedExternalCall, PinnedFixture};

use crate::walker::walk_calls;

/// Capture the Saturn 1 upgrade trace from a live archive RPC.
pub async fn capture_live(rpc_url: &str) -> Result<FrozenTrace> {
    let provider = ProviderBuilder::new().connect_http(rpc_url.parse()?);

    // Validate chain ID
    let chain_id = provider.get_chain_id().await.context("failed to get chain ID")?;
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
        .with_call_config(CallConfig { only_top_call: Some(false), with_log: Some(false) });

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

    Ok(FrozenTrace { pinned, effects, external_calls })
}
