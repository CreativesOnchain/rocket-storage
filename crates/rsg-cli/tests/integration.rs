//! Integration tests for the verdict engine.
//!
//! These tests use self-contained JSON fixtures (no network required) and
//! assert the correct `Verdict` for each scenario.

use rsg_compare::compare;
use rsg_types::{
    FrozenTrace, Manifest, ManifestEffect, ManifestExternalCall, ObservedEffect,
    PinnedFixture, RequirementLevel, StorageOp, Verdict,
};

// ─── Helper builders ──────────────────────────────────────────────────────────

fn base_pinned() -> PinnedFixture {
    PinnedFixture::default()
}

fn make_trace(effects: Vec<ObservedEffect>, external_calls: Vec<rsg_types::ObservedExternalCall>) -> FrozenTrace {
    FrozenTrace {
        pinned: base_pinned(),
        effects,
        external_calls,
    }
}

fn make_manifest(effects: Vec<ManifestEffect>, external_calls: Vec<ManifestExternalCall>) -> Manifest {
    Manifest {
        version: "1".into(),
        fixture: base_pinned(),
        effects,
        external_calls,
    }
}

fn addr_effect(path: &str, old: &str, new: &str) -> ObservedEffect {
    ObservedEffect {
        call_index: 0,
        caller: "0x5b3b5c76391662e56d0ff72f31b89c409316c8ba".into(),
        op: StorageOp::SetAddress,
        raw_key: format!("0x{}", "ab".repeat(32)),
        semantic_path: Some(path.into()),
        old_value: old.into(),
        new_value: new.into(),
    }
}

fn bool_effect(path: &str, old: &str, new: &str) -> ObservedEffect {
    ObservedEffect {
        call_index: 1,
        caller: "0x5b3b5c76391662e56d0ff72f31b89c409316c8ba".into(),
        op: StorageOp::SetBool,
        raw_key: format!("0x{}", "cd".repeat(32)),
        semantic_path: Some(path.into()),
        old_value: old.into(),
        new_value: new.into(),
    }
}

fn addr_entry(path: &str, old: &str, new: &str) -> ManifestEffect {
    ManifestEffect {
        semantic_path: path.into(),
        raw_key: None,
        op: StorageOp::SetAddress,
        requirement: RequirementLevel::Required,
        multiplicity: 1,
        expected_old_value: old.into(),
        expected_new_value: new.into(),
        source_anchor: "test".into(),
        rationale: "test".into(),
    }
}

fn bool_entry(path: &str, old: &str, new: &str) -> ManifestEffect {
    ManifestEffect {
        semantic_path: path.into(),
        raw_key: None,
        op: StorageOp::SetBool,
        requirement: RequirementLevel::Required,
        multiplicity: 1,
        expected_old_value: old.into(),
        expected_new_value: new.into(),
        source_anchor: "test".into(),
        rationale: "test".into(),
    }
}

// ─── Test: PASS on perfect match ─────────────────────────────────────────────

#[test]
fn pass_on_exact_match() {
    let trace = make_trace(
        vec![
            addr_effect("contract.addressrocketMegapoolDelegate", "0x0", "0xaabbcc"),
            bool_effect("contract.existsrocketMegapoolDelegate", "false", "true"),
        ],
        vec![],
    );
    let manifest = make_manifest(
        vec![
            addr_entry("contract.addressrocketMegapoolDelegate", "any", "any"),
            bool_entry("contract.existsrocketMegapoolDelegate", "false", "true"),
        ],
        vec![],
    );
    assert_eq!(compare(&trace, &manifest), Verdict::Pass);
}

// ─── Test 1: Undeclared write → FAIL ──────────────────────────────────────────

#[test]
fn fail_undeclared_write() {
    let trace = make_trace(
        vec![addr_effect(
            "contract.addressrocketMegapoolDelegate",
            "0x0",
            "0xaabbcc",
        )],
        vec![],
    );
    // Manifest has no entries → any write is undeclared
    let manifest = make_manifest(vec![], vec![]);
    assert!(matches!(compare(&trace, &manifest), Verdict::Fail { .. }));
}

// ─── Test 2: Wrong new value → FAIL ──────────────────────────────────────────

#[test]
fn fail_wrong_new_value() {
    let trace = make_trace(
        vec![addr_effect(
            "contract.addressrocketMegapoolDelegate",
            "0x0",
            "0xdeadbeef", // wrong
        )],
        vec![],
    );
    let manifest = make_manifest(
        vec![addr_entry(
            "contract.addressrocketMegapoolDelegate",
            "any",
            "0xaabbcc", // expected
        )],
        vec![],
    );
    assert!(matches!(compare(&trace, &manifest), Verdict::Fail { .. }));
}

// ─── Test 3: Missing required effect → FAIL ───────────────────────────────────

#[test]
fn fail_missing_required_effect() {
    let trace = make_trace(vec![], vec![]); // empty trace
    let manifest = make_manifest(
        vec![addr_entry(
            "contract.addressrocketMegapoolDelegate",
            "any",
            "any",
        )],
        vec![],
    );
    let verdict = compare(&trace, &manifest);
    assert!(matches!(verdict, Verdict::Fail { .. }));
}

// ─── Test 4: Swapped addresses → FAIL ────────────────────────────────────────

#[test]
fn fail_swapped_addresses() {
    let trace = make_trace(
        vec![
            {
                let mut e = addr_effect(
                    "contract.addressrocketMegapoolDelegate",
                    "0x0",
                    "0xbbbb",
                );
                e.call_index = 0;
                e
            },
            {
                let mut e = addr_effect(
                    "contract.addressrocketMegapoolBase",
                    "0x0",
                    "0xaaaa",
                );
                e.raw_key = format!("0x{}", "ef".repeat(32));
                e.call_index = 1;
                e
            },
        ],
        vec![],
    );
    let manifest = make_manifest(
        vec![
            addr_entry("contract.addressrocketMegapoolDelegate", "any", "0xaaaa"),
            addr_entry("contract.addressrocketMegapoolBase", "any", "0xbbbb"),
        ],
        vec![],
    );
    let verdict = compare(&trace, &manifest);
    assert!(matches!(verdict, Verdict::Fail { .. }), "should detect swap");
}

// ─── Test 5: Type drift → FAIL ────────────────────────────────────────────────

#[test]
fn fail_type_drift() {
    // Observe SetUint but manifest expects SetAddress for same path
    let trace = make_trace(
        vec![ObservedEffect {
            call_index: 0,
            caller: "0x5b3b5c76391662e56d0ff72f31b89c409316c8ba".into(),
            op: StorageOp::SetUint, // <-- wrong type
            raw_key: format!("0x{}", "ab".repeat(32)),
            semantic_path: Some("contract.addressrocketMegapoolDelegate".into()),
            old_value: "0".into(),
            new_value: "999".into(),
        }],
        vec![],
    );
    let manifest = make_manifest(
        vec![addr_entry("contract.addressrocketMegapoolDelegate", "any", "any")],
        vec![],
    );
    assert!(matches!(compare(&trace, &manifest), Verdict::Fail { .. }));
}

// ─── Test 6: Duplicate mutation → FAIL ───────────────────────────────────────

#[test]
fn fail_duplicate_mutation() {
    let e1 = bool_effect("contract.existsrocketMegapoolDelegate", "false", "true");
    let mut e2 = e1.clone();
    e2.call_index = 1;
    let trace = make_trace(vec![e1, e2], vec![]);
    // Manifest expects multiplicity = 1
    let manifest = make_manifest(
        vec![bool_entry(
            "contract.existsrocketMegapoolDelegate",
            "false",
            "true",
        )],
        vec![],
    );
    assert!(matches!(compare(&trace, &manifest), Verdict::Fail { .. }));
}

// ─── Test 7: Unexpected external call → FAIL ──────────────────────────────────

#[test]
fn fail_unexpected_external_call() {
    let trace = make_trace(
        vec![],
        vec![rsg_types::ObservedExternalCall {
            call_index: 0,
            from: "0x5b3b5c76391662e56d0ff72f31b89c409316c8ba".into(),
            to: "0x000000000000000000000000000000000000dead".into(),
            selector: "0xdeadbeef".into(),
            eth_value: "0".into(),
            success: true,
        }],
    );
    // Manifest has NO allowed external calls
    let manifest = make_manifest(vec![], vec![]);
    assert!(matches!(compare(&trace, &manifest), Verdict::Fail { .. }));
}

// ─── Test 8: UNKNOWN on undecodeable key ─────────────────────────────────────

#[test]
fn unknown_undecodeable_key() {
    let trace = make_trace(
        vec![ObservedEffect {
            call_index: 0,
            caller: "0x5b3b5c76391662e56d0ff72f31b89c409316c8ba".into(),
            op: StorageOp::SetUint,
            raw_key: format!("0x{}", "de".repeat(32)),
            semantic_path: None, // <-- not decoded
            old_value: "0".into(),
            new_value: "1".into(),
        }],
        vec![],
    );
    let manifest = make_manifest(vec![], vec![]);
    assert!(matches!(compare(&trace, &manifest), Verdict::Unknown { .. }));
}

// ─── Test 9: Wrong chain ID → FAIL ────────────────────────────────────────────

#[test]
fn fail_wrong_chain_id() {
    let mut trace = make_trace(vec![], vec![]);
    trace.pinned.chain_id = 5; // Goerli instead of mainnet

    let manifest = make_manifest(vec![], vec![]);
    // chain_id mismatch is caught by validate_pinned → FAIL
    assert!(matches!(compare(&trace, &manifest), Verdict::Fail { .. }));
}

// ─── Test 10: Omitted deletion → FAIL ────────────────────────────────────────

#[test]
fn fail_omitted_deletion() {
    // Observed DeleteUint with non-zero new value (deletion didn't clear)
    let trace = make_trace(
        vec![ObservedEffect {
            call_index: 0,
            caller: "0x5b3b5c76391662e56d0ff72f31b89c409316c8ba".into(),
            op: StorageOp::DeleteUint,
            raw_key: format!("0x{}", "ab".repeat(32)),
            semantic_path: Some("contract.addressrocketMegapoolDelegate".into()),
            old_value: "100".into(),
            new_value: "100".into(), // should be "0"
        }],
        vec![],
    );
    let manifest = make_manifest(
        vec![ManifestEffect {
            semantic_path: "contract.addressrocketMegapoolDelegate".into(),
            raw_key: None,
            op: StorageOp::DeleteUint,
            requirement: RequirementLevel::Required,
            multiplicity: 1,
            expected_old_value: "100".into(),
            expected_new_value: "0".into(),
            source_anchor: "test".into(),
            rationale: "test".into(),
        }],
        vec![],
    );
    assert!(matches!(compare(&trace, &manifest), Verdict::Fail { .. }));
}

// ─── Test 11: Reordered effects still PASS ───────────────────────────────────

#[test]
fn pass_reordered_effects() {
    // Effects in trace are in reverse order vs manifest — should still PASS
    let mut e1 = addr_effect("contract.addressrocketMegapoolDelegate", "0x0", "0xaaaa");
    e1.call_index = 0;
    let mut e2 = bool_effect("contract.existsrocketMegapoolDelegate", "false", "true");
    e2.call_index = 1;

    let trace = make_trace(vec![e2, e1], vec![]); // reversed order

    let manifest = make_manifest(
        vec![
            addr_entry("contract.addressrocketMegapoolDelegate", "any", "0xaaaa"),
            bool_entry("contract.existsrocketMegapoolDelegate", "false", "true"),
        ],
        vec![],
    );
    assert_eq!(compare(&trace, &manifest), Verdict::Pass);
}

// ─── Test 12: Adversarial Fixtures Serialization Validation ───────────────────

#[test]
fn test_all_adversarial_fixtures_deserialize() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/adversarial");
    let entries = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("failed to read {:?}: {e}", dir));

    let mut count = 0;
    for entry in entries {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let content = std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("failed to read {path:?}: {e}"));
            let trace: Result<FrozenTrace, _> = serde_json::from_str(&content);
            assert!(
                trace.is_ok(),
                "failed to parse fixture {path:?}: {:?}",
                trace.err()
            );
            count += 1;
        }
    }
    assert_eq!(count, 11, "expected 11 adversarial fixture files");
}
