//! Recursive CallFrame tree walker for EVM execution traces.

use alloy::{primitives::Address, providers::Provider, rpc::types::trace::geth::CallFrame};
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
    if let Some(effect) = try_capture_storage_effect(frame, provider, catalogue, *call_index).await
    {
        effects.push(effect);
    } else if let Some(external_call) = try_capture_external_call(frame, *call_index) {
        external_calls.push(external_call);
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

/// Inspect whether a call frame targets RocketStorage with a recognized storage mutator selector.
pub fn is_rocket_storage_mutation(to: Address, input: &[u8]) -> Option<StorageOp> {
    if to != ROCKET_STORAGE || input.len() < 4 {
        return None;
    }
    let sel = hex::encode(&input[0..4]);
    StorageOp::from_selector(&sel)
}

/// Inspect whether a call frame represents an outbound external call initiated by the upgrade contract.
pub fn is_upgrade_external_call(from: Address, to: Address, input: &[u8]) -> bool {
    from == UPGRADE_CONTRACT && to != ROCKET_STORAGE && to != Address::ZERO && input.len() >= 4
}

/// Extract and decode a RocketStorage mutation effect if the frame matches mutator criteria.
async fn try_capture_storage_effect<P: Provider>(
    frame: &CallFrame,
    provider: &P,
    catalogue: &KeyCatalogue,
    call_index: usize,
) -> Option<ObservedEffect> {
    let to_addr = frame.to.unwrap_or(Address::ZERO);
    let from_addr = frame.from;
    let input = frame.input.as_ref();

    let op = is_rocket_storage_mutation(to_addr, input)?;

    match decode_mutator_call(provider, &op, input).await {
        Ok((raw_key, old_val, new_val)) => {
            let semantic = catalogue
                .lookup_typed_hex(&raw_key, &op)
                .map(|s| s.to_string());
            Some(ObservedEffect {
                call_index,
                caller: format!("{from_addr:?}"),
                op,
                raw_key,
                semantic_path: semantic,
                old_value: old_val,
                new_value: new_val,
            })
        }
        Err(e) => {
            eprintln!("[rsg] WARN decode[{call_index}]: {e}");
            None
        }
    }
}

/// Extract an external call representation if the frame matches outbound criteria.
pub fn try_capture_external_call(
    frame: &CallFrame,
    call_index: usize,
) -> Option<ObservedExternalCall> {
    let to_addr = frame.to.unwrap_or(Address::ZERO);
    let from_addr = frame.from;
    let input = frame.input.as_ref();

    if !is_upgrade_external_call(from_addr, to_addr, input) {
        return None;
    }

    let selector = format!("0x{}", hex::encode(&input[0..4]));
    let eth_value = frame
        .value
        .map(|v| v.to_string())
        .unwrap_or_else(|| "0".into());

    Some(ObservedExternalCall {
        call_index,
        from: format!("{from_addr:?}"),
        to: format!("{to_addr:?}"),
        selector,
        eth_value,
        success: frame.error.is_none(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Bytes, address};

    #[test]
    fn test_is_rocket_storage_mutation() {
        let set_uint_sel = hex::decode(StorageOp::SetUint.selector_hex()).unwrap();
        let mut calldata = set_uint_sel;
        calldata.extend_from_slice(&[0u8; 32]);

        assert_eq!(
            is_rocket_storage_mutation(ROCKET_STORAGE, &calldata),
            Some(StorageOp::SetUint)
        );

        // Wrong address
        let other_addr = address!("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
        assert_eq!(is_rocket_storage_mutation(other_addr, &calldata), None);

        // Unknown selector
        assert_eq!(
            is_rocket_storage_mutation(ROCKET_STORAGE, &[0xde, 0xad, 0xbe, 0xef]),
            None
        );

        // Too short
        assert_eq!(
            is_rocket_storage_mutation(ROCKET_STORAGE, &[0xe2, 0xa4]),
            None
        );
    }

    #[test]
    fn test_is_upgrade_external_call() {
        let vault = address!("1111111111111111111111111111111111111111");
        let calldata = vec![0x12, 0x34, 0x56, 0x78];

        assert!(is_upgrade_external_call(UPGRADE_CONTRACT, vault, &calldata));

        // Call to RocketStorage should NOT be considered an external call
        assert!(!is_upgrade_external_call(
            UPGRADE_CONTRACT,
            ROCKET_STORAGE,
            &calldata
        ));

        // Call to zero address should NOT be considered an external call
        assert!(!is_upgrade_external_call(
            UPGRADE_CONTRACT,
            Address::ZERO,
            &calldata
        ));

        // Call from different caller should NOT be considered
        assert!(!is_upgrade_external_call(vault, vault, &calldata));
    }

    #[test]
    fn test_try_capture_external_call() {
        let vault = address!("1111111111111111111111111111111111111111");
        let frame = CallFrame {
            from: UPGRADE_CONTRACT,
            to: Some(vault),
            input: Bytes::from(vec![0xaa, 0xbb, 0xcc, 0xdd]),
            ..Default::default()
        };

        let captured = try_capture_external_call(&frame, 3).expect("should capture external call");
        assert_eq!(captured.call_index, 3);
        assert_eq!(captured.selector, "0xaabbccdd");
        assert_eq!(captured.eth_value, "0");
        assert!(captured.success);
    }
}
