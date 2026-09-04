//! Shared types for rocket-storage-gate.
//!
//! All domain types are defined here to avoid circular dependencies
//! between the other crates.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

// ─── Pinned fixture constants ────────────────────────────────────────────────

/// The chain-and-block parameters pinned for a specific upgrade verification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    /// Name and version of the EVM replay tool used (e.g. "cast/0.3.0").
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

// ─── Storage types ───────────────────────────────────────────────────────────

/// Which RocketStorage typed setter/deleter was called.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum StorageOp {
    #[serde(alias = "setAddress")]
    SetAddress,
    #[serde(alias = "setBool")]
    SetBool,
    #[serde(alias = "setBytes")]
    SetBytes,
    #[serde(alias = "setBytes32")]
    SetBytes32,
    #[serde(alias = "setInt")]
    SetInt,
    #[serde(alias = "setString")]
    SetString,
    #[serde(alias = "setUint")]
    SetUint,
    #[serde(alias = "deleteAddress")]
    DeleteAddress,
    #[serde(alias = "deleteBool")]
    DeleteBool,
    #[serde(alias = "deleteBytes")]
    DeleteBytes,
    #[serde(alias = "deleteBytes32")]
    DeleteBytes32,
    #[serde(alias = "deleteInt")]
    DeleteInt,
    #[serde(alias = "deleteString")]
    DeleteString,
    #[serde(alias = "deleteUint")]
    DeleteUint,
    #[serde(alias = "addUint")]
    AddUint,
    #[serde(alias = "subUint")]
    SubUint,
}

impl StorageOp {
    /// 4-byte selector (big-endian hex without 0x prefix).
    pub fn selector_hex(&self) -> &'static str {
        match self {
            StorageOp::SetAddress   => "ca446dd9",
            StorageOp::SetBool      => "abfdcced",
            StorageOp::SetBytes     => "2e28d084",
            StorageOp::SetBytes32   => "4e91db08",
            StorageOp::SetInt       => "3e49bed0",
            StorageOp::SetString    => "6e899550",
            StorageOp::SetUint      => "e2a4853a",
            StorageOp::DeleteAddress => "0e14a376",
            StorageOp::DeleteBool   => "2c62ff2d",
            StorageOp::DeleteBytes  => "616b59f6",
            StorageOp::DeleteBytes32 => "0b9adc57",
            StorageOp::DeleteInt    => "8c160095",
            StorageOp::DeleteString => "f6bb3cc4",
            StorageOp::DeleteUint   => "e2b202bf",
            StorageOp::AddUint      => "adb353dc",
            StorageOp::SubUint      => "ebb9d8c9",
        }
    }

    /// Return whether this op deletes a value (new_value will be zero/empty).
    pub fn is_delete(&self) -> bool {
        matches!(
            self,
            StorageOp::DeleteAddress
                | StorageOp::DeleteBool
                | StorageOp::DeleteBytes
                | StorageOp::DeleteBytes32
                | StorageOp::DeleteInt
                | StorageOp::DeleteString
                | StorageOp::DeleteUint
        )
    }

    /// Try to parse from the 4-byte hex selector string (without 0x).
    pub fn from_selector(sel: &str) -> Option<Self> {
        let sel = sel.trim_start_matches("0x").to_lowercase();
        match sel.as_str() {
            "ca446dd9" => Some(StorageOp::SetAddress),
            "abfdcced" => Some(StorageOp::SetBool),
            "2e28d084" => Some(StorageOp::SetBytes),
            "4e91db08" => Some(StorageOp::SetBytes32),
            "3e49bed0" => Some(StorageOp::SetInt),
            "6e899550" => Some(StorageOp::SetString),
            "e2a4853a" => Some(StorageOp::SetUint),
            "0e14a376" => Some(StorageOp::DeleteAddress),
            "2c62ff2d" => Some(StorageOp::DeleteBool),
            "616b59f6" => Some(StorageOp::DeleteBytes),
            "0b9adc57" => Some(StorageOp::DeleteBytes32),
            "8c160095" => Some(StorageOp::DeleteInt),
            "f6bb3cc4" => Some(StorageOp::DeleteString),
            "e2b202bf" => Some(StorageOp::DeleteUint),
            "adb353dc" => Some(StorageOp::AddUint),
            "ebb9d8c9" => Some(StorageOp::SubUint),
            _ => None,
        }
    }
}

// ─── Observed effect ─────────────────────────────────────────────────────────

/// A single observed mutation to RocketStorage, captured from the upgrade tx replay.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

// ─── External call ───────────────────────────────────────────────────────────

/// An external call made by the upgrade contract (e.g. an initializer).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

// ─── Frozen trace ────────────────────────────────────────────────────────────

/// The complete normalized observed trace, serialized to `frozen-trace.json`.
/// Must be hash-stable across repeated runs (no wall-clock timestamps).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrozenTrace {
    pub pinned: PinnedFixture,
    pub effects: Vec<ObservedEffect>,
    pub external_calls: Vec<ObservedExternalCall>,
}

// ─── Manifest ────────────────────────────────────────────────────────────────

/// Requirement level for a manifest entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RequirementLevel {
    Required,
    Optional,
}

/// One expected RocketStorage mutation, as authored from the source + RPIPs.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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

// ─── Verdict ─────────────────────────────────────────────────────────────────

/// A specific reason for a FAIL verdict.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FailReason {
    /// A required manifest entry was never observed.
    MissingRequiredEffect { semantic_path: String },
    /// An observed mutation is not in the manifest.
    UndeclaredWrite { raw_key: String, op: String, new_value: String },
    /// Observed old or new value does not match manifest expectation.
    WrongValue { semantic_path: String, field: String, expected: String, observed: String },
    /// The op (storage type) does not match.
    TypeDrift { raw_key: String, expected_op: String, observed_op: String },
    /// A key appeared more times than declared.
    DuplicateMutation { semantic_path: String, expected: usize, observed: usize },
    /// An expected deletion was not observed.
    OmittedDeletion { semantic_path: String },
    /// Two contract addresses appear to be swapped.
    SwappedAddress { path_a: String, path_b: String },
    /// An external call target, selector, value, or multiplicity is wrong.
    UnexpectedExternalCall {
        to: String,
        selector: String,
        reason: String,
    },
}

/// A specific reason for an UNKNOWN verdict.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UnknownReason {
    /// A raw key could not be decoded to a semantic path.
    UndecodeableKey { raw_key: String, op: String },
    /// The call shape is not a supported mutator form.
    UnsupportedCallShape { call_index: usize, selector: String },
    /// The trace is incomplete or sourced from an unexpected address.
    TraceIncomplete { detail: String },
}

/// The overall verdict.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "verdict", rename_all = "UPPERCASE")]
pub enum Verdict {
    Pass,
    Fail { reasons: Vec<FailReason> },
    Unknown { reasons: Vec<UnknownReason> },
}

impl Verdict {
    pub fn exit_code(&self) -> i32 {
        match self {
            Verdict::Pass => 0,
            Verdict::Fail { .. } => 1,
            Verdict::Unknown { .. } => 2,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Verdict::Pass => "PASS",
            Verdict::Fail { .. } => "FAIL",
            Verdict::Unknown { .. } => "UNKNOWN",
        }
    }
}

// ─── Attestation bundle ──────────────────────────────────────────────────────

/// Hashes that bind the attestation bundle to its inputs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationHashes {
    /// SHA-256 of the canonical JSON of `FrozenTrace` (normalized, no timestamps).
    pub observed_trace_sha256: String,
    /// SHA-256 of the canonical manifest YAML (or JSON).
    pub manifest_sha256: String,
    /// SHA-256 of the review-record JSON.
    pub review_record_sha256: String,
    /// The upgrade transaction hash (from pinned fixture).
    pub upgrade_tx: String,
    /// The Rocket Pool source commit (from pinned fixture).
    pub source_commit: String,
    /// The rsg tool version string.
    pub tool_version: String,
}

/// The complete machine-readable attestation bundle.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttestationBundle {
    /// Schema version ("1").
    pub version: String,
    /// When this attestation was generated (ISO-8601, excluded from normalized hash).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_at: Option<String>,
    /// Input hashes.
    pub hashes: AttestationHashes,
    /// The pinned fixture.
    pub pinned: PinnedFixture,
    /// The overall verdict.
    pub verdict: Verdict,
    /// Fail or unknown reasons (empty on PASS).
    pub reasons: Vec<serde_json::Value>,
    /// Observed effects summary (count per op type).
    pub effect_counts: IndexMap<String, usize>,
    /// Observation boundary notice.
    pub observation_boundary: String,
    /// Trust model disclaimer.
    pub disclaimer: String,
}

impl AttestationBundle {
    pub const OBSERVATION_BOUNDARY: &'static str =
        "This attestation covers only typed RocketStorage mutations and declared \
         external calls captured from the upgrade transaction replay. It does not \
         cover state changes inside externally called contracts, events, or any \
         other protocol invariants.";

    pub const DISCLAIMER: &'static str =
        "PASS means only that the replayed payload conforms to the reviewed manifest \
         within the documented observation boundary. This is not an audit, security \
         certificate, or proof that the upgrade is safe or correct in its entirety.";
}
