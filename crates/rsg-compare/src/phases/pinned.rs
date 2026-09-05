//! Phase 0: Pinned fixture validation.

use rsg_types::{FailReason, FrozenTrace, Manifest};

/// Validate that the pinned fixture in the trace matches the manifest fixture parameters.
pub fn validate_pinned(trace: &FrozenTrace, manifest: &Manifest, fails: &mut Vec<FailReason>) {
    if let Err(e) = validate_chain_id(trace.pinned.chain_id, manifest.fixture.chain_id) {
        fails.push(e);
    }
    if let Err(e) = validate_upgrade_tx(&trace.pinned.upgrade_tx, &manifest.fixture.upgrade_tx) {
        fails.push(e);
    }
    if let Err(e) =
        validate_source_commit(&trace.pinned.source_commit, &manifest.fixture.source_commit)
    {
        fails.push(e);
    }
}

/// Validate that the observed chain ID matches the expected fixture chain ID.
pub fn validate_chain_id(observed: u64, expected: u64) -> Result<(), FailReason> {
    if observed != expected {
        Err(FailReason::WrongValue {
            semantic_path: "pinned.chain_id".to_string(),
            field: "chain_id".to_string(),
            expected: expected.to_string(),
            observed: observed.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Validate that the observed upgrade transaction hash matches the expected transaction hash (case-insensitive).
pub fn validate_upgrade_tx(observed: &str, expected: &str) -> Result<(), FailReason> {
    if observed.to_lowercase() != expected.to_lowercase() {
        Err(FailReason::WrongValue {
            semantic_path: "pinned.upgrade_tx".to_string(),
            field: "upgrade_tx".to_string(),
            expected: expected.to_string(),
            observed: observed.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Validate that the observed source commit matches the expected source commit.
pub fn validate_source_commit(observed: &str, expected: &str) -> Result<(), FailReason> {
    if observed != expected {
        Err(FailReason::WrongValue {
            semantic_path: "pinned.source_commit".to_string(),
            field: "source_commit".to_string(),
            expected: expected.to_string(),
            observed: observed.to_string(),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsg_types::PinnedFixture;

    #[test]
    fn test_validate_chain_id() {
        assert!(validate_chain_id(1, 1).is_ok());

        let err = validate_chain_id(5, 1).unwrap_err();
        assert!(matches!(
            err,
            FailReason::WrongValue {
                ref semantic_path,
                ref expected,
                ref observed,
                ..
            } if semantic_path == "pinned.chain_id" && expected == "1" && observed == "5"
        ));
    }

    #[test]
    fn test_validate_upgrade_tx() {
        assert!(validate_upgrade_tx("0xABCD", "0xabcd").is_ok());

        let err = validate_upgrade_tx("0x1111", "0x2222").unwrap_err();
        assert!(matches!(
            err,
            FailReason::WrongValue {
                ref semantic_path,
                ref field,
                ..
            } if semantic_path == "pinned.upgrade_tx" && field == "upgrade_tx"
        ));
    }

    #[test]
    fn test_validate_source_commit() {
        assert!(validate_source_commit("abcdef", "abcdef").is_ok());

        let err = validate_source_commit("abcdef", "123456").unwrap_err();
        assert!(matches!(
            err,
            FailReason::WrongValue {
                ref semantic_path,
                ref field,
                ..
            } if semantic_path == "pinned.source_commit" && field == "source_commit"
        ));
    }

    #[test]
    fn test_validate_pinned_multiple_mismatches() {
        let trace = FrozenTrace {
            pinned: PinnedFixture {
                chain_id: 11155111,
                upgrade_tx: "0xdead".to_string(),
                source_commit: "observed_hash".to_string(),
                ..Default::default()
            },
            effects: vec![],
            external_calls: vec![],
        };

        let manifest = Manifest {
            version: "1".to_string(),
            fixture: PinnedFixture {
                chain_id: 1,
                upgrade_tx: "0xbeef".to_string(),
                source_commit: "expected_hash".to_string(),
                ..Default::default()
            },
            effects: vec![],
            external_calls: vec![],
        };

        let mut fails = Vec::new();
        validate_pinned(&trace, &manifest, &mut fails);
        assert_eq!(fails.len(), 3);
    }
}
