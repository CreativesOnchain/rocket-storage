//! Replayed execution trace models.

use crate::fixture::PinnedFixture;
use crate::storage_op::StorageOp;
use serde::{Deserialize, Serialize};

/// A single observed mutation to RocketStorage, captured from the upgrade tx replay.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservedEffect {
    /// Sequential call index (0-based) within the upgrade transaction.
    pub call_index: usize,
    /// Address of the contract that called RocketStorage (checksummed hex).
    pub caller: String,
    /// Which mutator was invoked.
    pub op: StorageOp,
    /// Raw 32-byte key, hex-encoded with 0x prefix.
    pub raw_key: String,
    /// Semantic meaning of the key, if known (e.g. "contract.address.rocketNodeDeposit").
    pub semantic_path: Option<String>,
    /// Value before the mutation (hex for addresses/bytes, decimal for uints/ints, "true"/"false" for bools).
    pub old_value: String,
    /// Value after the mutation.
    pub new_value: String,
}

/// An external call made by the upgrade contract (e.g. an initializer).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservedExternalCall {
    pub call_index: usize,
    /// Caller address.
    pub from: String,
    /// Target contract address.
    pub to: String,
    /// 4-byte selector, hex with 0x prefix.
    pub selector: String,
    /// ETH value sent (decimal string, usually "0").
    pub eth_value: String,
    /// Whether the call succeeded.
    pub success: bool,
}

/// The complete normalized observed trace, serialized to `frozen-trace.json`.
/// Must be hash-stable across repeated runs (no wall-clock timestamps).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FrozenTrace {
    pub pinned: PinnedFixture,
    pub effects: Vec<ObservedEffect>,
    pub external_calls: Vec<ObservedExternalCall>,
}
