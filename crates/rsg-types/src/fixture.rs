//! Pinned environment parameters for deterministic upgrade verification.

use serde::{Deserialize, Serialize};

/// The chain-and-block parameters pinned for a specific upgrade verification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PinnedFixture {
    /// Ethereum mainnet = 1
    pub chain_id: u64,
    /// Block just before the upgrade transaction was mined.
    pub pre_block: u64,
    /// Hash of `pre_block` (hex, 0x-prefixed).
    pub pre_block_hash: String,
    /// The upgrade transaction hash (hex, 0x-prefixed).
    pub upgrade_tx: String,
    /// Block in which the upgrade transaction was mined.
    pub exec_block: u64,
    /// Address of the upgrade proxy / executor contract (hex, 0x-prefixed).
    pub upgrade_contract: String,
    /// Address of the RocketStorage contract (hex, 0x-prefixed).
    pub rocket_storage: String,
    /// Git commit hash of the Rocket Pool source used to derive the manifest.
    pub source_commit: String,
    /// Name and version of the EVM replay tool used (e.g. "rsg-capture/0.1.0").
    pub replay_tool: String,
}

impl Default for PinnedFixture {
    fn default() -> Self {
        Self {
            chain_id: 1,
            pre_block: 24_479_993,
            pre_block_hash: String::new(),
            upgrade_tx: "0x2fc10aad3c1b00bdfa9b6fddab79e0f2688609848f8f7a1a6449ab42da38530c"
                .to_string(),
            exec_block: 24_479_994,
            upgrade_contract: "0x5b3b5c76391662e56d0ff72f31b89c409316c8ba".to_string(),
            rocket_storage: "0x1d8f8f00cfa6758d7be78336684788fb0ee0fa46".to_string(),
            source_commit: "fb7d9c428dc3dddc3fbd3e634e3cb365655df89e".to_string(),
            replay_tool: "rsg-capture/0.1.0".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinned_fixture_default_invariants() {
        let fixture = PinnedFixture::default();
        assert_eq!(fixture.chain_id, 1);
        assert_eq!(fixture.pre_block, 24_479_993);
        assert_eq!(fixture.exec_block, 24_479_994);
        assert_eq!(
            fixture.upgrade_tx,
            "0x2fc10aad3c1b00bdfa9b6fddab79e0f2688609848f8f7a1a6449ab42da38530c"
        );
        assert_eq!(
            fixture.rocket_storage,
            "0x1d8f8f00cfa6758d7be78336684788fb0ee0fa46"
        );
    }
}
