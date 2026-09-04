//! Comparator and verdict engine.
//!
//! Implements all failure checks described in the specification and returns a
//! deterministic `Verdict` with machine-readable `FailReason` and `UnknownReason` variants.

pub mod normalizer;
pub mod phases;

pub use normalizer::normalise_value;
pub use phases::{
    check_effect_against_entry, check_external_calls, check_manifest_effects,
    check_undeclared_writes, detect_swapped_addresses, scan_observed_effects, validate_pinned,
};

use rsg_types::{FailReason, FrozenTrace, Manifest, UnknownReason, Verdict};

/// Compare a `FrozenTrace` against a `Manifest` and return a verdict.
///
/// This is the core verification function of the tool. It is completely deterministic:
/// given the same inputs it always produces the same `Verdict`.
pub fn compare(trace: &FrozenTrace, manifest: &Manifest) -> Verdict {
    let mut fail_reasons: Vec<FailReason> = Vec::new();
    let mut unknown_reasons: Vec<UnknownReason> = Vec::new();

    // ── Check 0: Pinned fixture parameters must match ─────────────────────────
    validate_pinned(trace, manifest, &mut fail_reasons);

    // ── Phase 1: Scan observed effects & check for unresolvable keys ───────────
    let observed_counts = scan_observed_effects(&trace.effects, &mut unknown_reasons);

    // If any unknowns exist at this point, fail-closed immediately:
    // we cannot safely assert correctness when unresolvable keys are present.
    if !unknown_reasons.is_empty() {
        return Verdict::Unknown {
            reasons: unknown_reasons,
        };
    }

    // ── Phase 2: Check every REQUIRED manifest entry is observed ──────────────
    check_manifest_effects(manifest, &observed_counts, &mut fail_reasons);

    // ── Phase 3: Check for undeclared storage writes ──────────────────────────
    check_undeclared_writes(manifest, &observed_counts, &mut fail_reasons);

    // ── Phase 4: Check external calls against allowlist & multiplicity ────────
    check_external_calls(trace, manifest, &mut fail_reasons);

    // ── Phase 5: Swapped address detection ────────────────────────────────────
    detect_swapped_addresses(trace, manifest, &mut fail_reasons);

    // ── Produce final verdict ────────────────────────────────────────────────
    if !fail_reasons.is_empty() {
        Verdict::Fail {
            reasons: fail_reasons,
        }
    } else {
        Verdict::Pass
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsg_types::{
        ManifestEffect, ManifestExternalCall, ObservedEffect, ObservedExternalCall, PinnedFixture,
        RequirementLevel, StorageOp,
    };

    fn base_fixture() -> PinnedFixture {
        PinnedFixture::default()
    }

    fn make_trace(effects: Vec<ObservedEffect>) -> FrozenTrace {
        FrozenTrace {
            pinned: base_fixture(),
            effects,
            external_calls: vec![],
        }
    }

    fn make_manifest(effects: Vec<ManifestEffect>) -> Manifest {
        Manifest {
            version: "1".to_string(),
            fixture: base_fixture(),
            effects,
            external_calls: vec![],
        }
    }

    fn uint_effect(path: &str, raw_key: &str, old: &str, new: &str) -> ObservedEffect {
        ObservedEffect {
            call_index: 0,
            caller: "0xupgradecontract".to_string(),
            op: StorageOp::SetUint,
            raw_key: raw_key.to_string(),
            semantic_path: Some(path.to_string()),
            old_value: old.to_string(),
            new_value: new.to_string(),
        }
    }

    fn uint_entry(path: &str, old: &str, new: &str) -> ManifestEffect {
        ManifestEffect {
            semantic_path: path.to_string(),
            raw_key: None,
            op: StorageOp::SetUint,
            requirement: RequirementLevel::Required,
            multiplicity: 1,
            expected_old_value: old.to_string(),
            expected_new_value: new.to_string(),
            source_anchor: "test".to_string(),
            rationale: "test".to_string(),
        }
    }

    #[test]
    fn pass_on_matching_effects() {
        let trace = make_trace(vec![uint_effect(
            "protocol.version",
            &format!("0xdeadbeef{}", "0".repeat(56)),
            "3",
            "4",
        )]);
        let manifest = make_manifest(vec![uint_entry("protocol.version", "3", "4")]);
        assert_eq!(compare(&trace, &manifest), Verdict::Pass);
    }

    #[test]
    fn fail_on_wrong_new_value() {
        let trace = make_trace(vec![uint_effect(
            "protocol.version",
            &format!("0xdeadbeef{}", "0".repeat(56)),
            "3",
            "5", // wrong
        )]);
        let manifest = make_manifest(vec![uint_entry("protocol.version", "3", "4")]);
        let verdict = compare(&trace, &manifest);
        assert!(matches!(verdict, Verdict::Fail { .. }));
    }

    #[test]
    fn fail_on_missing_required_effect() {
        let trace = make_trace(vec![]); // empty
        let manifest = make_manifest(vec![uint_entry("protocol.version", "3", "4")]);
        let verdict = compare(&trace, &manifest);
        assert!(matches!(verdict, Verdict::Fail { .. }));
    }

    #[test]
    fn fail_on_undeclared_write() {
        let trace = make_trace(vec![uint_effect(
            "protocol.version",
            &format!("0xdeadbeef{}", "0".repeat(56)),
            "3",
            "4",
        )]);
        let manifest = make_manifest(vec![]);
        let verdict = compare(&trace, &manifest);
        assert!(matches!(verdict, Verdict::Fail { .. }));
    }

    #[test]
    fn unknown_on_undecodeable_key() {
        let trace = FrozenTrace {
            pinned: base_fixture(),
            effects: vec![ObservedEffect {
                call_index: 0,
                caller: "0xdeadbeef".to_string(),
                op: StorageOp::SetUint,
                raw_key: "0x".to_string() + &"de".repeat(32),
                semantic_path: None, // <-- undecodable
                old_value: "0".to_string(),
                new_value: "1".to_string(),
            }],
            external_calls: vec![],
        };
        let manifest = make_manifest(vec![]);
        let verdict = compare(&trace, &manifest);
        assert!(matches!(verdict, Verdict::Unknown { .. }));
    }

    #[test]
    fn fail_on_swapped_addresses() {
        let path_a = "contract.address[rocketVault]";
        let path_b = "contract.address[rocketTokenRETH]";
        let addr_a = "0x1111111111111111111111111111111111111111";
        let addr_b = "0x2222222222222222222222222222222222222222";

        // Swapped in trace
        let trace = FrozenTrace {
            pinned: base_fixture(),
            effects: vec![
                ObservedEffect {
                    call_index: 0,
                    caller: "0xcaller".to_string(),
                    op: StorageOp::SetAddress,
                    raw_key: "0xaa".to_string(),
                    semantic_path: Some(path_a.to_string()),
                    old_value: "0x0".to_string(),
                    new_value: addr_b.to_string(), // swapped!
                },
                ObservedEffect {
                    call_index: 1,
                    caller: "0xcaller".to_string(),
                    op: StorageOp::SetAddress,
                    raw_key: "0xbb".to_string(),
                    semantic_path: Some(path_b.to_string()),
                    old_value: "0x0".to_string(),
                    new_value: addr_a.to_string(), // swapped!
                },
            ],
            external_calls: vec![],
        };

        let manifest = Manifest {
            version: "1".to_string(),
            fixture: base_fixture(),
            effects: vec![
                ManifestEffect {
                    semantic_path: path_a.to_string(),
                    raw_key: None,
                    op: StorageOp::SetAddress,
                    requirement: RequirementLevel::Required,
                    multiplicity: 1,
                    expected_old_value: "any".to_string(),
                    expected_new_value: addr_a.to_string(),
                    source_anchor: "test".to_string(),
                    rationale: "test".to_string(),
                },
                ManifestEffect {
                    semantic_path: path_b.to_string(),
                    raw_key: None,
                    op: StorageOp::SetAddress,
                    requirement: RequirementLevel::Required,
                    multiplicity: 1,
                    expected_old_value: "any".to_string(),
                    expected_new_value: addr_b.to_string(),
                    source_anchor: "test".to_string(),
                    rationale: "test".to_string(),
                },
            ],
            external_calls: vec![],
        };

        let verdict = compare(&trace, &manifest);
        match verdict {
            Verdict::Fail { reasons } => {
                assert!(reasons
                    .iter()
                    .any(|r| matches!(r, FailReason::SwappedAddress { .. })));
            }
            _ => panic!("expected Fail with SwappedAddress, got {:?}", verdict),
        }
    }

    #[test]
    fn fail_on_unexpected_external_call() {
        let trace = FrozenTrace {
            pinned: base_fixture(),
            effects: vec![],
            external_calls: vec![ObservedExternalCall {
                call_index: 0,
                from: "0xcaller".to_string(),
                to: "0xrogue".to_string(),
                selector: "0xdeadbeef".to_string(),
                eth_value: "0".to_string(),
                success: true,
            }],
        };
        let manifest = make_manifest(vec![]);
        let verdict = compare(&trace, &manifest);
        match verdict {
            Verdict::Fail { reasons } => {
                assert!(reasons
                    .iter()
                    .any(|r| matches!(r, FailReason::UnexpectedExternalCall { .. })));
            }
            _ => panic!("expected Fail with UnexpectedExternalCall, got {:?}", verdict),
        }
    }

    #[test]
    fn pass_on_authorized_external_call() {
        let trace = FrozenTrace {
            pinned: base_fixture(),
            effects: vec![],
            external_calls: vec![ObservedExternalCall {
                call_index: 0,
                from: "0xcaller".to_string(),
                to: "0xvault".to_string(),
                selector: "0x12345678".to_string(),
                eth_value: "0".to_string(),
                success: true,
            }],
        };
        let mut manifest = make_manifest(vec![]);
        manifest.external_calls = vec![ManifestExternalCall {
            target: "0xvault".to_string(),
            selector: "0x12345678".to_string(),
            eth_value: "0".to_string(),
            multiplicity: 1,
            rationale: "test".to_string(),
        }];
        assert_eq!(compare(&trace, &manifest), Verdict::Pass);
    }
}
