//! Known contract names, upgrade addresses, and namespace constants.

/// Single-segment keys: `keccak256(abi.encodePacked(key))`.
pub const SINGLE_KEYS: &[&str] = &[
    "protocol.version",
    "protocol.version.string",
    "rocketpool.flag.one",
    "rocket.storage.version",
];

/// Known Rocket Pool contract names across Saturn 1 (v1.4) and historical releases.
pub const ALL_CONTRACT_NAMES: &[&str] = &[
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

/// Saturn 1 new contracts registered during the upgrade.
pub const SATURN1_NEW_CONTRACTS: &[(&str, &str)] = &[
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

/// Saturn 1 old contracts replaced and deregistered during the upgrade.
pub const SATURN1_OLD_CONTRACTS: &[(&str, &str)] = &[
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

/// Security council authorized parameters.
pub const SECURITY_ALLOWED_SETTINGS: &[(&str, &str)] = &[
    ("network", "network.node.commission.share.security.council.adder"),
    ("network", "network.submit.rewards.enabled"),
];

/// Protocol settings organized by namespace.
pub const PROTOCOL_SETTINGS: &[(&str, &[&str])] = &[
    (
        "deposit",
        &[
            "deposit.assign.socialised.maximum",
            "express.queue.rate",
            "express.queue.tickets.base.provision",
        ],
    ),
    (
        "network",
        &[
            "network.node.commission.share",
            "network.node.commission.share.security.council.adder",
            "network.voter.share",
            "network.pdao.share",
            "network.max.node.commission.share.council.adder",
            "network.max.reth.balance.delta",
        ],
    ),
    (
        "node",
        &[
            "reduced.bond",
            "node.unstaking.period",
            "node.withdrawal.cooldown",
            "node.minimum.legacy.staked.rpl",
            "node.per.minipool.stake.minimum",
            "node.per.minipool.stake.maximum",
            "node.deposit.enabled",
        ],
    ),
    ("minipool", &["minipool.maximum.penalty.count"]),
    ("security", &["upgrade.delay", "upgradeveto.quorum"]),
    ("proposals", &["proposal.quorum", "proposal.veto.quorum"]),
    (
        "megapool",
        &[
            "megapool.time.before.dissolve",
            "megapool.dissolve.penalty",
            "maximum.megapool.eth.penalty",
            "notify.threshold",
            "late.notify.fine",
            "user.distribute.delay",
            "user.distribute.delay.shortfall",
            "megapool.penalty.threshold",
            "deployed",
        ],
    ),
];

// ── Query Helpers ────────────────────────────────────────────────────────────

/// Find the registered address for a new Saturn 1 contract by contract name.
pub fn find_new_contract_address(name: &str) -> Option<&'static str> {
    SATURN1_NEW_CONTRACTS.iter().find(|(n, _)| *n == name).map(|(_, addr)| *addr)
}

/// Find the replaced address for an old Saturn 1 contract by contract name.
pub fn find_old_contract_address(name: &str) -> Option<&'static str> {
    SATURN1_OLD_CONTRACTS.iter().find(|(n, _)| *n == name).map(|(_, addr)| *addr)
}

/// Check if a contract name is in the list of known Rocket Pool contracts.
pub fn is_known_contract_name(name: &str) -> bool {
    ALL_CONTRACT_NAMES.contains(&name)
}

/// Check if a namespace and setting pair is permitted under DAO security rules.
pub fn is_security_allowed_setting(namespace: &str, setting: &str) -> bool {
    SECURITY_ALLOWED_SETTINGS.iter().any(|(ns, s)| *ns == namespace && *s == setting)
}

/// Check if a setting is registered under a given protocol setting namespace.
pub fn is_protocol_setting(namespace: &str, setting: &str) -> bool {
    PROTOCOL_SETTINGS.iter().any(|(ns, settings)| *ns == namespace && settings.contains(&setting))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hasher::parse_address_20;
    use std::collections::HashSet;

    #[test]
    fn test_saturn1_new_contracts_have_valid_20_byte_addresses() {
        for (name, addr_str) in SATURN1_NEW_CONTRACTS {
            let parsed = parse_address_20(addr_str);
            assert!(parsed.is_ok(), "invalid address for new contract {name}: {addr_str}");
        }
    }

    #[test]
    fn test_saturn1_old_contracts_have_valid_20_byte_addresses() {
        for (name, addr_str) in SATURN1_OLD_CONTRACTS {
            let parsed = parse_address_20(addr_str);
            assert!(parsed.is_ok(), "invalid address for old contract {name}: {addr_str}");
        }
    }

    #[test]
    fn test_no_duplicate_contract_names_in_registries() {
        let mut new_names = HashSet::new();
        for (name, _) in SATURN1_NEW_CONTRACTS {
            assert!(new_names.insert(*name), "duplicate in SATURN1_NEW_CONTRACTS: {name}");
        }

        let mut old_names = HashSet::new();
        for (name, _) in SATURN1_OLD_CONTRACTS {
            assert!(old_names.insert(*name), "duplicate in SATURN1_OLD_CONTRACTS: {name}");
        }

        let mut all_names = HashSet::new();
        for name in ALL_CONTRACT_NAMES {
            assert!(all_names.insert(*name), "duplicate in ALL_CONTRACT_NAMES: {name}");
        }
    }

    #[test]
    fn test_all_upgraded_contracts_are_in_all_contract_names() {
        for (name, _) in SATURN1_NEW_CONTRACTS {
            assert!(
                is_known_contract_name(name),
                "{name} from SATURN1_NEW_CONTRACTS missing from ALL_CONTRACT_NAMES"
            );
        }
        for (name, _) in SATURN1_OLD_CONTRACTS {
            assert!(
                is_known_contract_name(name),
                "{name} from SATURN1_OLD_CONTRACTS missing from ALL_CONTRACT_NAMES"
            );
        }
    }

    #[test]
    fn test_registry_lookup_helpers() {
        assert_eq!(
            find_new_contract_address("rocketMegapoolFactory"),
            Some("0xd5bffeaa9f373b9c367132772faa0b88e3f0e38b")
        );
        assert_eq!(find_new_contract_address("nonExistentContract"), None);

        assert_eq!(
            find_old_contract_address("rocketNodeManager"),
            Some("0x2b52479f6ea009907e46fc43e91064d1b92fdc86")
        );
        assert_eq!(find_old_contract_address("nonExistentContract"), None);

        assert!(is_known_contract_name("rocketStorage"));
        assert!(!is_known_contract_name("fakeContract"));

        assert!(is_security_allowed_setting("network", "network.submit.rewards.enabled"));
        assert!(!is_security_allowed_setting("network", "unknown.setting"));

        assert!(is_protocol_setting("deposit", "express.queue.rate"));
        assert!(!is_protocol_setting("deposit", "nonexistent.setting"));
    }
}
