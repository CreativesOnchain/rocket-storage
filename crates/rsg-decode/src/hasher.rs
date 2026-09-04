//! Keccak-256 hashing and byte parsing utilities for RocketStorage keys.

use sha3::{Digest, Keccak256};

/// Computes `keccak256(abi.encodePacked(parts...))` by sequentially feeding slices into Keccak-256.
pub fn keccak256_packed(parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    for part in parts {
        hasher.update(part);
    }
    hasher.finalize().into()
}

/// Parse a "0x..." or bare 64-hex-character string into a 32-byte array.
pub fn parse_bytes32(s: &str) -> anyhow::Result<[u8; 32]> {
    let s = s.trim_start_matches("0x");
    let bytes = hex::decode(s)?;
    if bytes.len() != 32 {
        anyhow::bail!("expected 32 bytes, got {}", bytes.len());
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

/// Parse a "0x..." or bare 40-hex-character string into a 20-byte address array.
pub fn parse_address_20(s: &str) -> anyhow::Result<[u8; 20]> {
    let s = s.trim_start_matches("0x");
    let bytes = hex::decode(s)?;
    if bytes.len() != 20 {
        anyhow::bail!("expected 20 bytes, got {}", bytes.len());
    }
    let mut arr = [0u8; 20];
    arr.copy_from_slice(&bytes);
    Ok(arr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_bytes32() {
        let hex_str = "0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20";
        let parsed = parse_bytes32(hex_str).unwrap();
        assert_eq!(parsed[0], 0x01);
        assert_eq!(parsed[31], 0x20);
    }

    #[test]
    fn parses_valid_address_20() {
        let addr_str = "0x1d8f8f00cfa6758d7be78336684788fb0ee0fa46";
        let parsed = parse_address_20(addr_str).unwrap();
        assert_eq!(parsed.len(), 20);
    }

    #[test]
    fn keccak256_packed_matches_solidity() {
        // keccak256(abi.encodePacked("protocol.version"))
        let hash = keccak256_packed(&[b"protocol.version"]);
        assert_eq!(
            hex::encode(hash),
            "7e9a8418e3cb3a35d39fd8f9eb7270d174a25d27a14e7c14cb1a96d475bb2bac"
        );
    }
}
