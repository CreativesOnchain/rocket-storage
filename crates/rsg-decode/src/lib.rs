//! Semantic key decoder for RocketStorage `bytes32` keys.
//!
//! RocketStorage keys are `keccak256(abi.encodePacked(...))`.
//! This crate maintains a catalogue of known keys and decodes raw bytes32
//! keys into human-readable semantic paths.

pub mod catalogue;
pub mod hasher;
pub mod registry;

pub use catalogue::KeyCatalogue;
pub use hasher::{keccak256_packed, parse_address_20, parse_bytes32};
pub use registry::{
    find_new_contract_address, find_old_contract_address, is_known_contract_name,
    is_protocol_setting, is_security_allowed_setting,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalogue_builds_without_panic() {
        let cat = KeyCatalogue::build();
        assert!(cat.len() >= 232, "catalogue should have at least 232 entries");
    }

    #[test]
    fn known_contract_address_key_decoded() {
        let cat = KeyCatalogue::build();
        let k = keccak256_packed(&[b"contract.address", b"rocketStorage"]);
        let hex_str = format!("0x{}", hex::encode(k));
        let res = cat.lookup_hex(&hex_str);
        assert_eq!(res, Some("contract.address.rocketStorage"));
    }

    #[test]
    fn unknown_key_returns_none() {
        let cat = KeyCatalogue::build();
        let unknown = [0xdeu8; 32];
        assert!(cat.lookup(&unknown).is_none());
    }
}
