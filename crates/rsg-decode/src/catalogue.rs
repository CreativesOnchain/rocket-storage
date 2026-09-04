//! Key catalogue lookup table for RocketStorage `bytes32` keys.

use crate::hasher::{keccak256_packed, parse_address_20, parse_bytes32};
use crate::registry::{
    ALL_CONTRACT_NAMES, PROTOCOL_SETTINGS, SATURN1_NEW_CONTRACTS, SATURN1_OLD_CONTRACTS,
    SECURITY_ALLOWED_SETTINGS, SINGLE_KEYS,
};
use rsg_types::StorageOp;
use std::collections::HashMap;

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
        let mut cat = Self { table: HashMap::new(), typed_table: HashMap::new() };

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
        // RocketMerkleDistributorMainnet relay address: keccak256(abi.encodePacked("rewards.relay.address", uint256(0)))
        let zero_u256 = [0u8; 32];
        let relay_key = keccak256_packed(&[b"rewards.relay.address", &zero_u256]);
        self.insert(relay_key, "rewards.relay.address.0".to_string());

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

        // RocketNetworkRevenues Universal Adjustable Revenue Split (UARS)
        let node_share_key = keccak256_packed(&[b"network.revenue.node.share"]);
        let voter_share_key = keccak256_packed(&[b"network.revenue.voter.share"]);
        let pdao_share_key = keccak256_packed(&[b"network.revenue.pdao.share"]);

        for (share_name, key_bytes) in
            [("node", &node_share_key), ("voter", &voter_share_key), ("pdao", &pdao_share_key)]
        {
            self.insert(*key_bytes, format!("network.revenue.{share_name}.share.checkpoint.0"));

            let length_key = keccak256_packed(&[b"snapshot.time.length", key_bytes]);
            self.insert(
                length_key,
                format!("snapshot.time.length.network.revenue.{share_name}.share"),
            );

            // Timestamped initial value: bytes32(uint256(key) + block.timestamp) at block 24,479,994 (timestamp 1771372799)
            let timestamp = 1771372799u128;
            let mut key_num = *key_bytes;
            let mut carry = timestamp;
            for i in (0..32).rev() {
                let sum = (key_num[i] as u128) + (carry & 0xff);
                key_num[i] = (sum & 0xff) as u8;
                carry = (carry >> 8) + (sum >> 8);
            }
            self.insert(key_num, format!("network.revenue.{share_name}.share.value.{timestamp}"));
        }
    }
}
