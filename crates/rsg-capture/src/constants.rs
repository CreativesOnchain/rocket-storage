//! Saturn 1 upgrade deployment constants.

use alloy::primitives::{Address, address};

/// RocketStorage contract address on Ethereum mainnet.
pub const ROCKET_STORAGE: Address = address!("1d8f8f00cfa6758d7be78336684788fb0ee0fa46");

/// Saturn 1 upgrade transaction hash on Ethereum mainnet.
pub const UPGRADE_TX: &str = "0x2fc10aad3c1b00bdfa9b6fddab79e0f2688609848f8f7a1a6449ab42da38530c";

/// Block number immediately prior to the Saturn 1 upgrade execution.
pub const PRE_BLOCK: u64 = 24_479_993;

/// Block number containing the Saturn 1 upgrade transaction.
pub const EXEC_BLOCK: u64 = 24_479_994;

/// Rocket Pool repository git commit hash associated with Saturn 1 release.
pub const SOURCE_COMMIT: &str = "fb7d9c428dc3dddc3fbd3e634e3cb365655df89e";

/// Temporary upgrade executor contract deployed for the Saturn 1 upgrade.
pub const UPGRADE_CONTRACT: Address = address!("5b3b5c76391662e56d0ff72f31b89c409316c8ba");
