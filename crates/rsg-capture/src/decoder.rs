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
    let (key_bytes, args) = extract_key_and_args(input)?;
    let raw_key = format!("0x{}", hex::encode(key_bytes));

    let old_val = read_pre_block_storage_value(provider, op, key_bytes.into()).await;
    let new_val = decode_new_value(op, args);

    Ok((raw_key, old_val, new_val))
}

/// Extract the 32-byte storage key and argument slice from mutator calldata.
pub fn extract_key_and_args(input: &[u8]) -> Result<([u8; 32], &[u8])> {
    let args = input.get(4..).context("calldata too short")?;
    let key_bytes: [u8; 32] = args
        .get(..32)
        .context("no key bytes")?
        .try_into()
        .map_err(|_| anyhow::anyhow!("invalid key length"))?;
    Ok((key_bytes, args))
}

/// Query a view method on RocketStorage at the pre-upgrade block.
async fn query_storage_view<P: Provider, C: SolCall>(provider: &P, call: C) -> Bytes {
    let calldata = Bytes::from(call.abi_encode());
    let block_id = BlockId::Number(BlockNumberOrTag::Number(PRE_BLOCK));

    provider
        .call(TransactionRequest::default().to(ROCKET_STORAGE).input(calldata.into()))
        .block(block_id)
        .await
        .unwrap_or_default()
}

/// Read the existing value in storage prior to the upgrade transaction.
async fn read_pre_block_storage_value<P: Provider>(
    provider: &P,
    op: &StorageOp,
    key: FixedBytes<32>,
) -> String {
    match op {
        StorageOp::SetAddress | StorageOp::DeleteAddress => {
            let raw =
                query_storage_view(provider, IRocketStorageRead::getAddressCall { _key: key })
                    .await;
            parse_address_result(&raw)
        }
        StorageOp::SetUint | StorageOp::AddUint | StorageOp::SubUint | StorageOp::DeleteUint => {
            let raw =
                query_storage_view(provider, IRocketStorageRead::getUintCall { _key: key }).await;
            parse_u256_result(&raw)
        }
        StorageOp::SetBool | StorageOp::DeleteBool => {
            let raw =
                query_storage_view(provider, IRocketStorageRead::getBoolCall { _key: key }).await;
            raw.last().map(|&b| (b != 0).to_string()).unwrap_or_else(|| "false".into())
        }
        StorageOp::SetBytes32 => {
            let raw =
                query_storage_view(provider, IRocketStorageRead::getBytes32Call { _key: key })
                    .await;
            format!("0x{}", hex::encode(&raw[..32.min(raw.len())]))
        }
        StorageOp::SetString | StorageOp::DeleteString => {
            let raw =
                query_storage_view(provider, IRocketStorageRead::getStringCall { _key: key }).await;
            parse_string_result(&raw)
        }
        StorageOp::SetBytes | StorageOp::DeleteBytes => {
            let raw =
                query_storage_view(provider, IRocketStorageRead::getBytesCall { _key: key }).await;
            parse_bytes_result(&raw)
        }
        _ => "unknown".into(),
    }
}

/// Decode the target new value for this mutation from the calldata arguments.
pub fn decode_new_value(op: &StorageOp, args: &[u8]) -> String {
    if op.is_delete() {
        return match op {
            StorageOp::DeleteAddress => "0x0000000000000000000000000000000000000000".into(),
            StorageOp::DeleteUint => "0".into(),
            StorageOp::DeleteBool => "false".into(),
            StorageOp::DeleteString => String::new(),
            StorageOp::DeleteBytes => "0x".into(),
            _ => "unknown".into(),
        };
    }

    match op {
        StorageOp::SetAddress => parse_address_from_args(args, 32),
        StorageOp::SetUint | StorageOp::AddUint | StorageOp::SubUint => args
            .get(32..64)
            .map(|b| U256::from_be_slice(b).to_string())
            .unwrap_or_else(|| "0".into()),
        StorageOp::SetBool => {
            args.get(63).map(|&b| (b != 0).to_string()).unwrap_or_else(|| "false".into())
        }
        StorageOp::SetBytes32 => args
            .get(32..64)
            .map(|b| format!("0x{}", hex::encode(b)))
            .unwrap_or_else(|| format!("0x{}", "0".repeat(64))),
        StorageOp::SetString => parse_string_from_args(args, 32),
        StorageOp::SetBytes => parse_bytes_from_args(args, 32),
        _ => "unknown".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_key_and_args() {
        let mut calldata = vec![0x12, 0x34, 0x56, 0x78]; // selector
        let key = [0xabu8; 32];
        calldata.extend_from_slice(&key);
        calldata.extend_from_slice(&[0x00; 32]); // extra arg

        let (extracted_key, args) = extract_key_and_args(&calldata).unwrap();
        assert_eq!(extracted_key, key);
        assert_eq!(args.len(), 64);
    }

    #[test]
    fn test_extract_key_too_short() {
        let short = vec![0x12, 0x34];
        assert!(extract_key_and_args(&short).is_err());
    }

    #[test]
    fn test_decode_new_value_for_deletions() {
        assert_eq!(
            decode_new_value(&StorageOp::DeleteAddress, &[]),
            "0x0000000000000000000000000000000000000000"
        );
        assert_eq!(decode_new_value(&StorageOp::DeleteUint, &[]), "0");
        assert_eq!(decode_new_value(&StorageOp::DeleteBool, &[]), "false");
        assert_eq!(decode_new_value(&StorageOp::DeleteString, &[]), "");
        assert_eq!(decode_new_value(&StorageOp::DeleteBytes, &[]), "0x");
    }

    #[test]
    fn test_decode_new_value_for_set_uint_and_bool() {
        let mut args = vec![0u8; 64]; // key (32) + uint value (32)
        args[63] = 42;

        assert_eq!(decode_new_value(&StorageOp::SetUint, &args), "42");
        assert_eq!(decode_new_value(&StorageOp::SetBool, &args), "true");

        args[63] = 0;
        assert_eq!(decode_new_value(&StorageOp::SetBool, &args), "false");
    }
}
