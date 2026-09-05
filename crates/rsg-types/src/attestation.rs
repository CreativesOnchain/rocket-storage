//! Attestation bundle and cryptographic hashes.

use crate::fixture::PinnedFixture;
use crate::verdict::Verdict;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Hashes that bind the attestation bundle to its inputs.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    /// Standard observation boundary disclosure.
    pub const OBSERVATION_BOUNDARY: &'static str = "This attestation covers only typed RocketStorage mutations and declared \
         external calls captured from the upgrade transaction replay. It does not \
         cover state changes inside externally called contracts, events, or any \
         other protocol invariants.";

    /// Standard non-audit disclaimer.
    pub const DISCLAIMER: &'static str = "PASS means only that the replayed payload conforms to the reviewed manifest \
         within the documented observation boundary. This is not an audit, security \
         certificate, or proof that the upgrade is safe or correct in its entirety.";
}
