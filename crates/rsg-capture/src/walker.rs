//! Recursive CallFrame tree walker for EVM execution traces.

use alloy::{
    primitives::Address,
    providers::Provider,
    rpc::types::trace::geth::CallFrame,
};
use anyhow::Result;
use rsg_decode::KeyCatalogue;
use rsg_types::{ObservedEffect, ObservedExternalCall, StorageOp};

use crate::{
    constants::{ROCKET_STORAGE, UPGRADE_CONTRACT},
    decoder::decode_mutator_call,
};

/// Recursively walk geth CallTracer frames, extracting RocketStorage mutations
/// and external calls initiated by the upgrade contract.
pub async fn walk_calls<P>(
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
                    let semantic = catalogue
                        .lookup_typed_hex(&raw_key, &op)
                        .map(|s| s.to_string());
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
        let eth_value = frame
            .value
            .map(|v| v.to_string())
            .unwrap_or_else(|| "0".into());
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
