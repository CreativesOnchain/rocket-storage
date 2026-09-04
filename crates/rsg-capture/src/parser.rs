//! Calldata and return data parsing utilities for EVM ABI types.

use alloy::primitives::U256;

/// Parse an EVM address returned from an ABI view call (right-aligned in 32 bytes).
pub fn parse_address_result(raw: &[u8]) -> String {
    if raw.len() >= 32 {
        format!("0x{}", hex::encode(&raw[12..32]))
    } else {
        "0x0000000000000000000000000000000000000000".into()
    }
}

/// Parse an EVM address parameter from ABI-encoded function arguments at the specified offset.
pub fn parse_address_from_args(args: &[u8], offset: usize) -> String {
    if args.len() >= offset + 32 {
        format!("0x{}", hex::encode(&args[offset + 12..offset + 32]))
    } else {
        "0x0000000000000000000000000000000000000000".into()
    }
}

/// Parse a uint256 returned from an ABI view call.
pub fn parse_u256_result(raw: &[u8]) -> String {
    if raw.len() >= 32 {
        U256::from_be_slice(&raw[..32]).to_string()
    } else {
        "0".into()
    }
}

/// Parse dynamic ABI-encoded string from function arguments at the specified offset.
pub fn parse_string_from_args(args: &[u8], offset_in_args: usize) -> String {
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

/// Parse dynamic ABI-encoded string returned from an ABI view call.
pub fn parse_string_result(raw: &[u8]) -> String {
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

/// Parse dynamic ABI-encoded bytes from function arguments at the specified offset.
pub fn parse_bytes_from_args(args: &[u8], offset_in_args: usize) -> String {
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

/// Parse dynamic ABI-encoded bytes returned from an ABI view call.
pub fn parse_bytes_result(raw: &[u8]) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_address() {
        let mut raw = vec![0u8; 32];
        raw[31] = 0x42;
        let addr = parse_address_result(&raw);
        assert_eq!(addr, "0x0000000000000000000000000000000000000042");

        let empty = parse_address_result(&[]);
        assert_eq!(empty, "0x0000000000000000000000000000000000000000");
    }

    #[test]
    fn test_parse_u256() {
        let mut raw = vec![0u8; 32];
        raw[31] = 100;
        let val = parse_u256_result(&raw);
        assert_eq!(val, "100");

        let empty = parse_u256_result(&[]);
        assert_eq!(empty, "0");
    }

    #[test]
    fn test_parse_string_result() {
        let mut raw = vec![0u8; 32]; // offset 0x20
        raw[31] = 0x20;
        let mut len_bytes = vec![0u8; 32];
        len_bytes[31] = 5;
        let data = b"hello";
        let mut full = raw;
        full.extend_from_slice(&len_bytes);
        full.extend_from_slice(data);
        full.resize(96, 0);

        let parsed = parse_string_result(&full);
        assert_eq!(parsed, "hello");
    }
}
