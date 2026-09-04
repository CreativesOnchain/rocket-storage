//! Semantic key decoder for RocketStorage `bytes32` keys.
//!
//! RocketStorage keys are `keccak256(abi.encodePacked(...))`.
//! This crate maintains a catalogue of known keys and decodes raw bytes32
//! keys into human-readable semantic paths.

use sha3::{Digest, Keccak256};
use std::collections::HashMap;

/// Lookup table: `bytes32` key (32-byte array) → semantic path string.
pub struct KeyCatalogue {
    table: HashMap<[u8; 32], String>,
    typed_table: HashMap<([u8; 32], rsg_types::StorageOp), String>,
}

impl Default for KeyCatalogue {
    fn default() -> Self {
        Self::build()
    }
}

impl KeyCatalogue {
    /// Build the catalogue from the well-known Rocket Pool key patterns.
    pub fn build() -> Self {
        let mut cat = Self {
            table: HashMap::new(),
            typed_table: HashMap::new(),
        };

        // 1. Single-segment keys
        for key_str in SINGLE_KEYS {
            let hash = keccak256_packed(&[key_str.as_bytes()]);
            cat.insert(hash, key_str.to_string());
        }

        // 2. All contract names (contract.address & contract.abi)
        for name in ALL_CONTRACT_NAMES {
            let k_addr = keccak256_packed(&[b"contract.address", name.as_bytes()]);
            cat.insert(k_addr, format!("contract.address.{name}"));

            let k_abi = keccak256_packed(&[b"contract.abi", name.as_bytes()]);
            cat.insert(k_abi, format!("contract.abi.{name}"));
        }

        // 3. New contract address registrations (contract.exists & contract.name with 20 raw bytes)
        for (name, addr_str) in SATURN1_NEW_CONTRACTS {
            if let Ok(addr) = parse_address_20(addr_str) {
                let k_exists = keccak256_packed(&[b"contract.exists", &addr]);
                cat.insert(k_exists, format!("contract.exists.{name}"));

                let k_name = keccak256_packed(&[b"contract.name", &addr]);
                cat.insert(k_name, format!("contract.name.{name}"));
            }
        }

        // 4. Old contract address deregistrations (contract.exists.old & contract.name.old)
        for (name, addr_str) in SATURN1_OLD_CONTRACTS {
            if let Ok(addr) = parse_address_20(addr_str) {
                let k_exists = keccak256_packed(&[b"contract.exists", &addr]);
                cat.insert(k_exists, format!("contract.exists.old.{name}"));

                let k_name = keccak256_packed(&[b"contract.name", &addr]);
                cat.insert(k_name, format!("contract.name.old.{name}"));
            }
        }

        // 5. Security Council Allowed Settings
        for (ns, name) in SECURITY_ALLOWED_SETTINGS {
            let k = keccak256_packed(&[b"dao.security.allowed.setting", ns.as_bytes(), name.as_bytes()]);
            cat.insert(k, format!("dao.security.allowed.setting.{ns}.{name}"));
        }

        // 6. Protocol Settings across all namespaces
        for (ns, settings) in PROTOCOL_SETTINGS {
            let ns_hash = keccak256_packed(&[b"dao.protocol.setting.", ns.as_bytes()]);
            for setting in *settings {
                let k = keccak256_packed(&[&ns_hash, setting.as_bytes()]);
                cat.insert(k, format!("dao.protocol.setting.{ns}.{setting}"));
            }
        }

        // 7. RocketMerkleDistributorMainnet relay address: keccak256(abi.encodePacked("rewards.relay.address", uint256(0)))
        let zero_u256 = [0u8; 32];
        let relay_key = keccak256_packed(&[b"rewards.relay.address", &zero_u256]);
        cat.insert(relay_key, "rewards.relay.address.0".to_string());

        // 8. RocketMegapoolFactory delegate set: keccak256(abi.encodePacked("megapool.delegate.set"))
        // Disambiguate between addressStorage (delegate 0) and uintStorage (queue metadata)
        let delegate_set_key = keccak256_packed(&[b"megapool.delegate.set"]);
        cat.insert_typed(delegate_set_key, rsg_types::StorageOp::SetAddress, "megapool.delegate.set.delegate.0".to_string());
        cat.insert_typed(delegate_set_key, rsg_types::StorageOp::SetUint, "megapool.delegate.set.meta".to_string());
        cat.insert(delegate_set_key, "megapool.delegate.set".to_string());

        // 9. RocketNetworkRevenues Universal Adjustable Revenue Split (UARS)
        let node_share_key = keccak256_packed(&[b"network.revenue.node.share"]);
        let voter_share_key = keccak256_packed(&[b"network.revenue.voter.share"]);
        let pdao_share_key = keccak256_packed(&[b"network.revenue.pdao.share"]);

        for (share_name, key_bytes) in [
            ("node", &node_share_key),
            ("voter", &voter_share_key),
            ("pdao", &pdao_share_key),
        ] {
            // Checkpoint 0: bytes32(uint256(_key) + 0)
            cat.insert(*key_bytes, format!("network.revenue.{share_name}.share.checkpoint.0"));

            // Snapshot length: keccak256(abi.encodePacked("snapshot.time.length", _key))
            let length_key = keccak256_packed(&[b"snapshot.time.length", key_bytes]);
            cat.insert(length_key, format!("snapshot.time.length.network.revenue.{share_name}.share"));

            // Timestamped initial value: bytes32(uint256(key) + block.timestamp) at block 24,479,994 (timestamp 1771372799)
            let timestamp = 1771372799u128;
            let mut key_num = *key_bytes;
            let mut carry = timestamp;
            for i in (0..32).rev() {
                let sum = (key_num[i] as u128) + (carry & 0xff);
                key_num[i] = (sum & 0xff) as u8;
                carry = (carry >> 8) + (sum >> 8);
            }
            cat.insert(key_num, format!("network.revenue.{share_name}.share.value.{timestamp}"));
        }

        cat
    }

    /// Insert a generic mapping into the catalogue.
    pub fn insert(&mut self, key: [u8; 32], path: String) {
        self.table.insert(key, path);
    }

    /// Insert a type-specific mapping into the catalogue.
    pub fn insert_typed(&mut self, key: [u8; 32], op: rsg_types::StorageOp, path: String) {
        self.typed_table.insert((key, op), path);
    }

    /// Look up a raw bytes32 key with its StorageOp.
    pub fn lookup_typed(&self, key_bytes: &[u8; 32], op: &rsg_types::StorageOp) -> Option<&str> {
        if let Some(path) = self.typed_table.get(&(*key_bytes, *op)) {
            return Some(path.as_str());
        }
        self.lookup(key_bytes)
    }

    /// Look up a raw bytes32 key with its StorageOp from hex string.
    pub fn lookup_typed_hex(&self, hex: &str, op: &rsg_types::StorageOp) -> Option<&str> {
        let bytes = parse_bytes32(hex).ok()?;
        self.lookup_typed(&bytes, op)
    }

    /// Look up a raw bytes32 key without type disambiguation.
    ///
    /// Returns `Some(semantic_path)` if known, `None` if not in the catalogue.
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

    /// Return true if catalogue is empty.
    pub fn is_empty(&self) -> bool {
        self.table.is_empty()
    }
}

// ─── keccak256 helper ────────────────────────────────────────────────────────

/// `keccak256(abi.encodePacked(parts...))` — concatenate then hash.
pub fn keccak256_packed(parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    for part in parts {
        hasher.update(part);
    }
    hasher.finalize().into()
}

/// Parse "0x..." or bare 64-hex-char string into 32 bytes.
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

/// Parse "0x..." or bare 40-hex-char string into 20 bytes.
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

// ─── Known Constants & Contract Registry ────────────────────────────────────

const SINGLE_KEYS: &[&str] = &[
    "protocol.version",
    "protocol.version.string",
    "rocketpool.flag.one",
    "rocket.storage.version",
];

const ALL_CONTRACT_NAMES: &[&str] = &[
    // Saturn 1 (v1.4) 35 contracts:
    "rocketMegapoolDelegate",
    "rocketMegapoolFactory",
    "rocketMegapoolProxy",
    "rocketMegapoolManager",
    "linkedListStorage",
    "rocketDAOProtocolSettingsMegapool",
    "rocketDAOSecurityUpgrade",
    "rocketNetworkRevenues",
    "beaconStateVerifier",
    "rocketMegapoolPenalties",
    "rocketNetworkSnapshotsTime",
    "rocketNodeManager",
    "rocketNodeDeposit",
    "rocketNodeStaking",
    "rocketDepositPool",
    "rocketDAOProtocol",
    "rocketDAOProtocolProposals",
    "rocketDAOProtocolSettingsNode",
    "rocketDAOProtocolSettingsDeposit",
    "rocketDAOProtocolSettingsNetwork",
    "rocketDAOProtocolSettingsSecurity",
    "rocketDAOProtocolSettingsMinipool",
    "rocketDAOSecurityProposals",
    "rocketDAONodeTrustedUpgrade",
    "rocketNetworkBalances",
    "rocketNetworkSnapshots",
    "rocketNetworkPenalties",
    "rocketRewardsPool",
    "rocketNodeDistributorDelegate",
    "rocketClaimDAO",
    "rocketMinipoolBondReducer",
    "rocketMinipoolManager",
    "rocketNetworkVoting",
    "rocketMerkleDistributorMainnet",
    "rocketDAOProtocolSettingsProposals",
    // Core and historical contracts:
    "rocketStorage",
    "rocketVault",
    "rocketTokenRETH",
    "rocketTokenRPL",
    "rocketTokenRPLFixedSupply",
    "rocketSmoothingPool",
    "rocketClaimNode",
    "rocketClaimTrustedNode",
    "rocketNodeDistributor",
    "rocketNodeDistributorFactory",
    "rocketMinipoolBase",
    "rocketMinipoolDelegate",
    "rocketMinipoolFactory",
    "rocketMinipoolQueue",
    "rocketMinipoolStatus",
    "rocketMinipoolPenalty",
    "rocketMegapoolBase",
    "rocketMegapoolDelegateOld",
    "rocketDAOProtocolActions",
    "rocketDAOProtocolProposal",
    "rocketDAOProtocolVerifier",
    "rocketDAOProtocolSettingsAuction",
    "rocketDAOProtocolSettingsInflation",
    "rocketDAOProtocolSettingsRewards",
    "rocketDAONodeTrusted",
    "rocketDAONodeTrustedActions",
    "rocketDAONodeTrustedProposals",
    "rocketDAONodeTrustedSettingsMembers",
    "rocketDAONodeTrustedSettingsProposals",
    "rocketDAONodeTrustedSettingsRewards",
    "rocketNetworkFees",
    "rocketNetworkPrices",
    "rocketDAOSecurity",
    "rocketDAOSecurityActions",
    "rocketUpgradeOneDotTwo",
    "rocketUpgradeOneDotThree",
    "rocketUpgradeOneDotFour",
];

const SATURN1_NEW_CONTRACTS: &[(&str, &str)] = &[
    ("rocketMegapoolDelegate", "0xca3dd4bee7c174903dbf66c3897c27e9adaaebdd"),
    ("rocketMegapoolFactory", "0xd5bffeaa9f373b9c367132772faa0b88e3f0e38b"),
    ("rocketMegapoolProxy", "0x1b389d76a04d01026c5f5b0a125d4ccf26f9cd51"),
    ("rocketMegapoolManager", "0xf2ccd522ba5ffeda28fe0389963845d61f342034"),
    ("linkedListStorage", "0x52590e8aac140e2020f8f51695719922ebccb6d6"),
    ("rocketDAOProtocolSettingsMegapool", "0x40628faac22383327b9f7bbc86cd1857050a2dce"),
    ("rocketDAOSecurityUpgrade", "0x950baf0358164339114914169bf16754789b5dc4"),
    ("rocketNetworkRevenues", "0x9d9708da8e0200dd8dd9ad09e0aaf184ad260842"),
    ("beaconStateVerifier", "0xe9a114c50f26001443b91079ab5573a90d2d8469"),
    ("rocketMegapoolPenalties", "0xa2afc3c2d8ea4ebdbe925cade17c29517630e6ab"),
    ("rocketNetworkSnapshotsTime", "0x569f5b3024054ab4049a50df223a747afe18a891"),
    ("rocketNodeManager", "0xcf2d76a7499d3acb5a22ce83c027651e8d76e250"),
    ("rocketNodeDeposit", "0x6b13698c306a297fee1383cdc2c65d63781d2d47"),
    ("rocketNodeStaking", "0xedfc7dcae43ff954577a2875a9d805874490ee3e"),
    ("rocketDepositPool", "0xce15294273cfb9d9b628f4d61636623decdf4fdc"),
    ("rocketDAOProtocol", "0xcac25e88276a333cf9d4196d112d93af67ef809a"),
    ("rocketDAOProtocolProposals", "0xcf7f6e23cd8189b7f56b14f66e11241c8ac0f03b"),
    ("rocketDAOProtocolSettingsNode", "0xb02b883303e658ddcd58d3871dc4ca0c91f0fc9d"),
    ("rocketDAOProtocolSettingsDeposit", "0x227be8dd01df8ad9bed0178e4f8cec2996c5c365"),
    ("rocketDAOProtocolSettingsNetwork", "0x67fd03a5095197d1ad1f932bc55e022c420b1153"),
    ("rocketDAOProtocolSettingsSecurity", "0xc9d771aaf504f33bb3c8a7e67ea9f1881f837cff"),
    ("rocketDAOProtocolSettingsMinipool", "0xaef94c3650aa13d7a2456477fc374a16b94b9152"),
    ("rocketDAOSecurityProposals", "0x334b9b1a6f9d7531efb13746482ff40f1c2a0c4e"),
    ("rocketDAONodeTrustedUpgrade", "0x9290aa076a2f1418a4e414e3d83ae03ca8e1ad10"),
    ("rocketNetworkBalances", "0x1d9f14c6bfd8358b589964bad8665add248e9473"),
    ("rocketNetworkSnapshots", "0xe37f2d9dfb7397caf671df5190a5dfb601028f17"),
    ("rocketNetworkPenalties", "0xed0493de30e82be7c16c8925c7204ce9d1136b3a"),
    ("rocketRewardsPool", "0xcba5951fc706fc783b7c142dae8576ebe29c41fd"),
    ("rocketNodeDistributorDelegate", "0x35a85d4c115801395e6e3abaa784fb05826f129d"),
    ("rocketClaimDAO", "0xfb2f2ab63dcf412ced6cde5f4f809215ed0c81aa"),
    ("rocketMinipoolBondReducer", "0xde8ab526b19fca2d5a57c4a78b698041717be591"),
    ("rocketMinipoolManager", "0xe54b8c641fd96de5d6747f47c19964c6b824d62c"),
    ("rocketNetworkVoting", "0x994a9c49230fec0c127b8f42d6c5288f02610aed"),
    ("rocketMerkleDistributorMainnet", "0xe4e2612ee8d7fdc8518faea85770a3b9c886e2f5"),
    ("rocketDAOProtocolSettingsProposals", "0xf6ad771dfb1cd10c66f688e251b5e5c21cbfdf81"),
];

const SATURN1_OLD_CONTRACTS: &[(&str, &str)] = &[
    ("rocketNodeManager", "0x2b52479f6ea009907e46fc43e91064d1b92fdc86"),
    ("rocketNodeDeposit", "0x672335b91b4f2096d897ca1b12ef4ec9346a5ff4"),
    ("rocketNodeStaking", "0xf18dc176c10ff6d8b5a17974126d43301f8eeb95"),
    ("rocketDepositPool", "0xdd3f50f8a6cafbe9b31a427582963f465e745af8"),
    ("rocketDAOProtocol", "0x1b714ed0ce30a8bedc5b4253daaa08c84ca5bfcb"),
    ("rocketDAOProtocolProposals", "0x6d736da1dc2562dbea9998385a0a27d8c2b2793e"),
    ("rocketDAOProtocolSettingsNode", "0x448da008c7eb2501165c9aa62dffeec4405bc660"),
    ("rocketDAOProtocolSettingsDeposit", "0xd846aa34caef083dc4797d75096f60b6e08b7418"),
    ("rocketDAOProtocolSettingsNetwork", "0x89682e5f9bf69c909fc5e21a06495ac35e3671ab"),
    ("rocketDAOProtocolSettingsSecurity", "0x1ec364cdd9697f56b8cb17a745b98c2b862cbe29"),
    ("rocketDAOProtocolSettingsMinipool", "0xa416a7a07925d60f794e20532bc730749611a220"),
    ("rocketDAOSecurityProposals", "0x6004fa90a27db9971add200d1a3bb34444db9fb7"),
    ("rocketDAONodeTrustedUpgrade", "0x952999ec97248547d810fd6464fdb78855b022ab"),
    ("rocketNetworkBalances", "0x6cc65bf618f55ce2433f9d8d827fc44117d81399"),
    ("rocketNetworkSnapshots", "0x7603352f1c4752ac07aac94e48632b65fdf1d35c"),
    ("rocketNetworkPenalties", "0x9294fc6f03c64cc217f5be8697ea3ed2de77e2f8"),
    ("rocketRewardsPool", "0xee4d2a71cf479e0d3d0c3c2c923dbfeb57e73111"),
    ("rocketNodeDistributorDelegate", "0x32778d6bf5b93b89177d328556eeeb35c09f472b"),
    ("rocketClaimDAO", "0xfe6db0ce3f61a4ae04c0a3e62f775a6f511c9aac"),
    ("rocketMinipoolBondReducer", "0xf7ab34c74c02407ed653ac9128731947187575c0"),
    ("rocketMinipoolManager", "0xf82991bd8976c243eb3b7cddc52ab0fc8dc1246c"),
    ("rocketNetworkVoting", "0x77cf0f32bdd06242465eb3318a81196194a13daa"),
    ("rocketMerkleDistributorMainnet", "0x5ce71e603b138f7e65029cc1918c0566ed0dbd4b"),
    ("rocketDAOProtocolSettingsProposals", "0x59cd103df1be2ebd80d45c54a3cde8d4f812c034"),
];

const SECURITY_ALLOWED_SETTINGS: &[(&str, &str)] = &[
    ("network", "network.node.commission.share.security.council.adder"),
    ("network", "network.submit.rewards.enabled"),
];

const PROTOCOL_SETTINGS: &[(&str, &[&str])] = &[
    ("deposit", &[
        "deposit.assign.socialised.maximum",
        "express.queue.rate",
        "express.queue.tickets.base.provision",
    ]),
    ("network", &[
        "network.node.commission.share",
        "network.node.commission.share.security.council.adder",
        "network.voter.share",
        "network.pdao.share",
        "network.max.node.commission.share.council.adder",
        "network.max.reth.balance.delta",
    ]),
    ("node", &[
        "reduced.bond",
        "node.unstaking.period",
        "node.withdrawal.cooldown",
        "node.minimum.legacy.staked.rpl",
        "node.per.minipool.stake.minimum",
        "node.per.minipool.stake.maximum",
        "node.deposit.enabled",
    ]),
    ("minipool", &[
        "minipool.maximum.penalty.count",
    ]),
    ("security", &[
        "upgrade.delay",
        "upgradeveto.quorum",
    ]),
    ("proposals", &[
        "proposal.quorum",
        "proposal.veto.quorum",
    ]),
    ("megapool", &[
        "megapool.time.before.dissolve",
        "megapool.dissolve.penalty",
        "maximum.megapool.eth.penalty",
        "notify.threshold",
        "late.notify.fine",
        "user.distribute.delay",
        "user.distribute.delay.shortfall",
        "megapool.penalty.threshold",
        "deployed",
    ]),
];

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
