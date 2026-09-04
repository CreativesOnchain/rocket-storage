//! Expected-effects manifest models.

use crate::fixture::PinnedFixture;
use crate::storage_op::StorageOp;
use serde::{Deserialize, Serialize};

/// Requirement level for a manifest entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequirementLevel {
    Required,
    Optional,
}

/// One expected RocketStorage mutation, as authored from the source + RPIPs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManifestEffect {
    /// Human-readable name (e.g. "contract.address.rocketNodeDeposit").
    pub semantic_path: String,
    /// The raw 32-byte key (hex). May be omitted if `semantic_path` is in the catalogue.
    pub raw_key: Option<String>,
    /// Which mutator is expected.
    pub op: StorageOp,
    /// Required or optional.
    pub requirement: RequirementLevel,
    /// How many times this mutation is expected to occur (usually 1).
    pub multiplicity: usize,
    /// Expected value BEFORE the mutation. Use "any" to skip old-value checking.
    pub expected_old_value: String,
    /// Expected value AFTER the mutation.
    pub expected_new_value: String,
    /// Source file + line reference anchoring this expectation.
    pub source_anchor: String,
    /// Human-readable rationale.
    pub rationale: String,
}

/// One expected external call made by the upgrade contract.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManifestExternalCall {
    /// Target contract address (checksummed hex).
    pub target: String,
    /// 4-byte selector (hex with 0x prefix). Use "any" to match any selector.
    pub selector: String,
    /// ETH value (decimal string).
    pub eth_value: String,
    /// Expected number of calls.
    pub multiplicity: usize,
    pub rationale: String,
}

/// The complete manifest: expected effects + external calls, authored before trace reconciliation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Manifest {
    /// Schema version (currently "1").
    pub version: String,
    /// Pinned fixture parameters this manifest was authored against.
    pub fixture: PinnedFixture,
    /// Ordered list of expected RocketStorage mutations.
    pub effects: Vec<ManifestEffect>,
    /// Allowed external calls.
    pub external_calls: Vec<ManifestExternalCall>,
}
