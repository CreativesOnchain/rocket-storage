//! Phase 0: Pinned fixture validation.

use rsg_types::{FailReason, FrozenTrace, Manifest};

/// Validate that the pinned fixture in the trace matches the manifest fixture parameters.
pub fn validate_pinned(trace: &FrozenTrace, manifest: &Manifest, fails: &mut Vec<FailReason>) {
    if trace.pinned.chain_id != manifest.fixture.chain_id {
        fails.push(FailReason::WrongValue {
            semantic_path: "pinned.chain_id".to_string(),
            field: "chain_id".to_string(),
            expected: manifest.fixture.chain_id.to_string(),
            observed: trace.pinned.chain_id.to_string(),
        });
    }
    if trace.pinned.upgrade_tx.to_lowercase() != manifest.fixture.upgrade_tx.to_lowercase() {
        fails.push(FailReason::WrongValue {
            semantic_path: "pinned.upgrade_tx".to_string(),
            field: "upgrade_tx".to_string(),
            expected: manifest.fixture.upgrade_tx.clone(),
            observed: trace.pinned.upgrade_tx.clone(),
        });
    }
    if trace.pinned.source_commit != manifest.fixture.source_commit {
        fails.push(FailReason::WrongValue {
            semantic_path: "pinned.source_commit".to_string(),
            field: "source_commit".to_string(),
            expected: manifest.fixture.source_commit.clone(),
            observed: trace.pinned.source_commit.clone(),
        });
    }
}
