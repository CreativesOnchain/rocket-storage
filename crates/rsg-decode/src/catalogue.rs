//! Key catalogue lookup table for RocketStorage `bytes32` keys.

use std::collections::HashMap;

use rsg_types::StorageOp;

use crate::hasher::{keccak256_packed, parse_address_20, parse_bytes32};
use crate::registry::{
    ALL_CONTRACT_NAMES, PROTOCOL_SETTINGS, SATURN1_NEW_CONTRACTS, SATURN1_OLD_CONTRACTS,
    SECURITY_ALLOWED_SETTINGS, SINGLE_KEYS,
};

/// Lookup table: `bytes32` key (32-byte array) → semantic path string.
#[derive(Debug, Clone)]
pub struct KeyCatalogue {
    table: HashMap<[u8; 32], String>,
    typed_table: HashMap<([u8; 32], StorageOp), String>,
}

impl Default for KeyCatalogue {
    fn default() -> Self {
        Self::build()
    }
}

impl KeyCatalogue {
    /// Build the catalogue pre-populated with all well-known Rocket Pool protocol keys.
    pub fn build() -> Self {
        let mut cat = Self {
            table: HashMap::new(),
            typed_table: HashMap::new(),
        };

        cat.register_singles();
        cat.register_contract_names();
        cat.register_new_contracts();
        cat.register_old_contracts();
        cat.register_security_settings();
        cat.register_protocol_settings();
        cat.register_special_keys();

        cat
    }

    /// Insert a generic mapping into the catalogue.
    pub fn insert(&mut self, key: [u8; 32], path: String) {
        self.table.insert(key, path);
    }

    /// Insert a type-specific mapping into the catalogue.
    pub fn insert_typed(&mut self, key: [u8; 32], op: StorageOp, path: String) {
        self.typed_table.insert((key, op), path);
    }

    /// Look up a raw bytes32 key with its StorageOp.
    pub fn lookup_typed(&self, key_bytes: &[u8; 32], op: &StorageOp) -> Option<&str> {
        if let Some(path) = self.typed_table.get(&(*key_bytes, *op)) {
            return Some(path.as_str());
        }
        self.lookup(key_bytes)
    }

    /// Look up a raw bytes32 key with its StorageOp from hex string.
    pub fn lookup_typed_hex(&self, hex: &str, op: &StorageOp) -> Option<&str> {
        let bytes = parse_bytes32(hex).ok()?;
        self.lookup_typed(&bytes, op)
    }

    /// Look up a raw bytes32 key without type disambiguation.
    pub fn lookup(&self, key_bytes: &[u8; 32]) -> Option<&str> {
        self.table.get(key_bytes).map(|s| s.as_str())
    }

    /// Look up from a hex string ("0x..." or bare hex).
    pub fn lookup_hex(&self, hex: &str) -> Option<&str> {
        let bytes = parse_bytes32(hex).ok()?;
        self.lookup(&bytes)
    }

    /// Return the total number of entries in the catalogue.
    pub fn len(&self) -> usize {
        self.table.len()
    }

    /// Return true if the catalogue is empty.
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }

    // ── Internal Builder Subroutines ─────────────────────────────────────────

    fn register_singles(&mut self) {
        for key_str in SINGLE_KEYS {
            let hash = keccak256_packed(&[key_str.as_bytes()]);
            self.insert(hash, key_str.to_string());
        }
    }

    fn register_contract_names(&mut self) {
        for name in ALL_CONTRACT_NAMES {
            let k_addr = keccak256_packed(&[b"contract.address", name.as_bytes()]);
            self.insert(k_addr, format!("contract.address.{name}"));

            let k_abi = keccak256_packed(&[b"contract.abi", name.as_bytes()]);
            self.insert(k_abi, format!("contract.abi.{name}"));
        }
    }

    fn register_new_contracts(&mut self) {
        for (name, addr_str) in SATURN1_NEW_CONTRACTS {
            if let Ok(addr) = parse_address_20(addr_str) {
                let k_exists = keccak256_packed(&[b"contract.exists", &addr]);
                self.insert(k_exists, format!("contract.exists.{name}"));

                let k_name = keccak256_packed(&[b"contract.name", &addr]);
                self.insert(k_name, format!("contract.name.{name}"));
            }
        }
    }

    fn register_old_contracts(&mut self) {
        for (name, addr_str) in SATURN1_OLD_CONTRACTS {
            if let Ok(addr) = parse_address_20(addr_str) {
                let k_exists = keccak256_packed(&[b"contract.exists", &addr]);
                self.insert(k_exists, format!("contract.exists.old.{name}"));

                let k_name = keccak256_packed(&[b"contract.name", &addr]);
                self.insert(k_name, format!("contract.name.old.{name}"));
            }
        }
    }

    fn register_security_settings(&mut self) {
        for (ns, name) in SECURITY_ALLOWED_SETTINGS {
            let k = keccak256_packed(&[
                b"dao.security.allowed.setting",
                ns.as_bytes(),
                name.as_bytes(),
            ]);
            self.insert(k, format!("dao.security.allowed.setting.{ns}.{name}"));
        }
    }

    fn register_protocol_settings(&mut self) {
        for (ns, settings) in PROTOCOL_SETTINGS {
            let ns_hash = keccak256_packed(&[b"dao.protocol.setting.", ns.as_bytes()]);
            for setting in *settings {
                let k = keccak256_packed(&[&ns_hash, setting.as_bytes()]);
                self.insert(k, format!("dao.protocol.setting.{ns}.{setting}"));
            }
        }
    }

    fn register_special_keys(&mut self) {
        self.register_relay_address_key();
        self.register_delegate_set_keys();
        self.register_uars_revenue_keys();
    }

    fn register_relay_address_key(&mut self) {
        // RocketMerkleDistributorMainnet relay address: keccak256(abi.encodePacked("rewards.relay.address", uint256(0)))
        let zero_u256 = [0u8; 32];
        let relay_key = keccak256_packed(&[b"rewards.relay.address", &zero_u256]);
        self.insert(relay_key, "rewards.relay.address.0".to_string());
    }

    fn register_delegate_set_keys(&mut self) {
        // RocketMegapoolFactory delegate set: keccak256(abi.encodePacked("megapool.delegate.set"))
        let delegate_set_key = keccak256_packed(&[b"megapool.delegate.set"]);
        self.insert_typed(
            delegate_set_key,
            StorageOp::SetAddress,
            "megapool.delegate.set.delegate.0".to_string(),
        );
        self.insert_typed(
            delegate_set_key,
            StorageOp::SetUint,
            "megapool.delegate.set.meta".to_string(),
        );
        self.insert(delegate_set_key, "megapool.delegate.set".to_string());
    }

    fn register_uars_revenue_keys(&mut self) {
        // RocketNetworkRevenues Universal Adjustable Revenue Split (UARS)
        let node_share_key = keccak256_packed(&[b"network.revenue.node.share"]);
        let voter_share_key = keccak256_packed(&[b"network.revenue.voter.share"]);
        let pdao_share_key = keccak256_packed(&[b"network.revenue.pdao.share"]);

        const SATURN1_BLOCK_TIMESTAMP: u128 = 1_771_372_799;

        for (share_name, key_bytes) in [
            ("node", &node_share_key),
            ("voter", &voter_share_key),
            ("pdao", &pdao_share_key),
        ] {
            self.register_uars_share_keys(share_name, key_bytes, SATURN1_BLOCK_TIMESTAMP);
        }
    }

    fn register_uars_share_keys(
        &mut self,
        share_name: &str,
        key_bytes: &[u8; 32],
        timestamp: u128,
    ) {
        self.insert(
            *key_bytes,
            format!("network.revenue.{share_name}.share.checkpoint.0"),
        );

        let length_key = keccak256_packed(&[b"snapshot.time.length", key_bytes]);
        self.insert(
            length_key,
            format!("snapshot.time.length.network.revenue.{share_name}.share"),
        );

        // Timestamped initial value: bytes32(uint256(key) + block.timestamp) at block 24,479,994
        let timestamped_key = add_u128_to_bytes32(key_bytes, timestamp);
        self.insert(
            timestamped_key,
            format!("network.revenue.{share_name}.share.value.{timestamp}"),
        );
    }
}

/// Add a `u128` offset to a big-endian 32-byte array, propagating carry across byte boundaries.
pub fn add_u128_to_bytes32(key: &[u8; 32], offset: u128) -> [u8; 32] {
    let mut result = *key;
    let mut carry = offset;
    for i in (0..32).rev() {
        let sum = (result[i] as u128) + (carry & 0xff);
        result[i] = (sum & 0xff) as u8;
        carry = (carry >> 8) + (sum >> 8);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_u128_to_bytes32() {
        let base = [0u8; 32];
        let res = add_u128_to_bytes32(&base, 100);
        assert_eq!(res[31], 100);
        for byte in &res[..31] {
            assert_eq!(*byte, 0);
        }

        // Test carry propagation
        let mut near_overflow = [0u8; 32];
        near_overflow[31] = 0xff;
        let with_carry = add_u128_to_bytes32(&near_overflow, 1);
        assert_eq!(with_carry[31], 0x00);
        assert_eq!(with_carry[30], 0x01);
    }

    #[test]
    fn test_insert_and_lookup_typed() {
        let mut cat = KeyCatalogue {
            table: HashMap::new(),
            typed_table: HashMap::new(),
        };
        let key = [0x42u8; 32];

        cat.insert(key, "generic.path".to_string());
        cat.insert_typed(key, StorageOp::SetAddress, "typed.address.path".to_string());
        cat.insert_typed(key, StorageOp::SetUint, "typed.uint.path".to_string());

        assert_eq!(cat.lookup(&key), Some("generic.path"));
        assert_eq!(
            cat.lookup_typed(&key, &StorageOp::SetAddress),
            Some("typed.address.path")
        );
        assert_eq!(
            cat.lookup_typed(&key, &StorageOp::SetUint),
            Some("typed.uint.path")
        );
        // Fallback to generic when type is not registered
        assert_eq!(
            cat.lookup_typed(&key, &StorageOp::SetBool),
            Some("generic.path")
        );
    }

    #[test]
    fn test_lookup_typed_hex() {
        let mut cat = KeyCatalogue {
            table: HashMap::new(),
            typed_table: HashMap::new(),
        };
        let key = [0xabu8; 32];
        cat.insert_typed(key, StorageOp::SetBytes, "typed.bytes".to_string());

        let hex_str = format!("0x{}", hex::encode(key));
        assert_eq!(
            cat.lookup_typed_hex(&hex_str, &StorageOp::SetBytes),
            Some("typed.bytes")
        );
        assert_eq!(
            cat.lookup_typed_hex("invalid_hex", &StorageOp::SetBytes),
            None
        );
    }

    #[test]
    fn test_len_and_is_empty() {
        let mut cat = KeyCatalogue {
            table: HashMap::new(),
            typed_table: HashMap::new(),
        };
        assert!(cat.is_empty());
        assert_eq!(cat.len(), 0);

        cat.insert([1u8; 32], "entry".to_string());
        assert!(!cat.is_empty());
        assert_eq!(cat.len(), 1);
    }

    #[test]
    fn test_special_keys_registration() {
        let cat = KeyCatalogue::build();
        let delegate_set_key = keccak256_packed(&[b"megapool.delegate.set"]);

        assert_eq!(
            cat.lookup_typed(&delegate_set_key, &StorageOp::SetAddress),
            Some("megapool.delegate.set.delegate.0")
        );
        assert_eq!(
            cat.lookup_typed(&delegate_set_key, &StorageOp::SetUint),
            Some("megapool.delegate.set.meta")
        );
    }
}
