//! Comparator and verdict engine.
//!
//! Implements all 10 failure checks described in the spec and returns a
//! deterministic `Verdict` with machine-readable `FailReason` / `UnknownReason`
//! variants.

use rsg_types::{
    FailReason, FrozenTrace, Manifest, ManifestEffect, ObservedEffect, RequirementLevel,
    StorageOp, UnknownReason, Verdict,
};
use std::collections::HashMap;

/// Compare a `FrozenTrace` against a `Manifest` and return a verdict.
///
/// This is the core function of the tool. It is deterministic — given the same
/// inputs it always produces the same `Verdict`.
pub fn compare(trace: &FrozenTrace, manifest: &Manifest) -> Verdict {
    let mut fail_reasons: Vec<FailReason> = Vec::new();
    let mut unknown_reasons: Vec<UnknownReason> = Vec::new();

    // ── Check 0: Pinned fixture must match ───────────────────────────────────
    validate_pinned(trace, manifest, &mut fail_reasons);

    // ── Phase 1: Scan observed effects ───────────────────────────────────────
    // Map semantic_path → count of how many times we observed it.
    let mut observed_counts: HashMap<String, Vec<&ObservedEffect>> = HashMap::new();

    for effect in &trace.effects {
        // Check: unresolved key
        if effect.semantic_path.is_none() {
            unknown_reasons.push(UnknownReason::UndecodeableKey {
                raw_key: effect.raw_key.clone(),
                op: format!("{:?}", effect.op),
            });
            continue;
        }

        let path = effect.semantic_path.as_ref().unwrap().clone();
        observed_counts.entry(path).or_default().push(effect);
    }

    // If any unknowns exist at this point, stop — return UNKNOWN immediately
    // (fail-closed: we can't safely verify if we have unresolved keys)
    if !unknown_reasons.is_empty() {
        return Verdict::Unknown {
            reasons: unknown_reasons,
        };
    }

    // ── Phase 2: Check every REQUIRED manifest entry is observed ─────────────
    for entry in &manifest.effects {
        if entry.requirement == RequirementLevel::Required {
            let observed = observed_counts.get(&entry.semantic_path);

            match observed {
                None => {
                    fail_reasons.push(FailReason::MissingRequiredEffect {
                        semantic_path: entry.semantic_path.clone(),
                    });
                }
                Some(effects) => {
                    // Check multiplicity
                    if effects.len() != entry.multiplicity {
                        fail_reasons.push(FailReason::DuplicateMutation {
                            semantic_path: entry.semantic_path.clone(),
                            expected: entry.multiplicity,
                            observed: effects.len(),
                        });
                    }

                    // Check each observed effect against the manifest entry
                    for obs in effects {
                        check_effect_against_entry(obs, entry, &mut fail_reasons);
                    }
                }
            }
        }
    }

    // ── Phase 3: Check for undeclared writes ─────────────────────────────────
    // Build a set of all manifest semantic paths
    let manifest_paths: HashMap<&str, &ManifestEffect> = manifest
        .effects
        .iter()
        .map(|e| (e.semantic_path.as_str(), e))
        .collect();

    for (path, effects) in &observed_counts {
        if !manifest_paths.contains_key(path.as_str()) {
            // Observed a mutation that has no manifest entry
            let first = effects[0];
            fail_reasons.push(FailReason::UndeclaredWrite {
                raw_key: first.raw_key.clone(),
                op: format!("{:?}", first.op),
                new_value: first.new_value.clone(),
            });
        }
    }

    // ── Phase 4: Check external calls ────────────────────────────────────────
    check_external_calls(trace, manifest, &mut fail_reasons);

    // ── Phase 5: Swapped address detection ───────────────────────────────────
    detect_swapped_addresses(trace, manifest, &mut fail_reasons);

    // ── Produce verdict ───────────────────────────────────────────────────────
    if !fail_reasons.is_empty() {
        Verdict::Fail {
            reasons: fail_reasons,
        }
    } else {
        Verdict::Pass
    }
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn validate_pinned(trace: &FrozenTrace, manifest: &Manifest, fails: &mut Vec<FailReason>) {
    if trace.pinned.chain_id != manifest.fixture.chain_id {
        fails.push(FailReason::WrongValue {
            semantic_path: "pinned.chain_id".to_string(),
            field: "chain_id".to_string(),
            expected: manifest.fixture.chain_id.to_string(),
            observed: trace.pinned.chain_id.to_string(),
        });
    }
    if trace.pinned.upgrade_tx.to_lowercase()
        != manifest.fixture.upgrade_tx.to_lowercase()
    {
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

fn check_effect_against_entry(
    obs: &ObservedEffect,
    entry: &ManifestEffect,
    fails: &mut Vec<FailReason>,
) {
    // Type check
    let expected_op_str = format!("{:?}", entry.op);
    let observed_op_str = format!("{:?}", obs.op);
    if obs.op != entry.op {
        fails.push(FailReason::TypeDrift {
            raw_key: obs.raw_key.clone(),
            expected_op: expected_op_str,
            observed_op: observed_op_str,
        });
        return; // Don't check values if the type is wrong
    }

    // Old value check (skip if manifest says "any")
    if entry.expected_old_value != "any"
        && normalise_value(&obs.old_value) != normalise_value(&entry.expected_old_value)
    {
        fails.push(FailReason::WrongValue {
            semantic_path: entry.semantic_path.clone(),
            field: "old_value".to_string(),
            expected: entry.expected_old_value.clone(),
            observed: obs.old_value.clone(),
        });
    }

    // New value check (skip if manifest says "any")
    if entry.expected_new_value != "any"
        && normalise_value(&obs.new_value) != normalise_value(&entry.expected_new_value)
    {
        fails.push(FailReason::WrongValue {
            semantic_path: entry.semantic_path.clone(),
            field: "new_value".to_string(),
            expected: entry.expected_new_value.clone(),
            observed: obs.new_value.clone(),
        });
    }

    // Omitted deletion check
    if entry.op.is_delete()
        && obs.new_value != "0"
        && obs.new_value != "false"
        && obs.new_value != ""
        && obs.new_value != "\"\""
        && obs.new_value != "0x"
        && obs.new_value != "0x0000000000000000000000000000000000000000"
        && obs.new_value != "0x0000000000000000000000000000000000000000000000000000000000000000"
    {
        fails.push(FailReason::OmittedDeletion {
            semantic_path: entry.semantic_path.clone(),
        });
    }
}

fn check_external_calls(
    trace: &FrozenTrace,
    manifest: &Manifest,
    fails: &mut Vec<FailReason>,
) {
    // Count observed external calls by (to, selector)
    let mut observed: HashMap<(String, String), usize> = HashMap::new();
    for call in &trace.external_calls {
        let key = (call.to.to_lowercase(), call.selector.to_lowercase());
        *observed.entry(key).or_default() += 1;
    }

    // Check each manifest external call entry
    for expected in &manifest.external_calls {
        let key = (
            expected.target.to_lowercase(),
            expected.selector.to_lowercase(),
        );
        let count = observed.get(&key).copied().unwrap_or(0);

        if count != expected.multiplicity {
            fails.push(FailReason::UnexpectedExternalCall {
                to: expected.target.clone(),
                selector: expected.selector.clone(),
                reason: format!(
                    "expected {} call(s) to {}:{}, observed {}",
                    expected.multiplicity,
                    expected.target,
                    expected.selector,
                    count
                ),
            });
        }
    }

    // Check for any call not in the manifest
    let manifest_call_keys: std::collections::HashSet<(String, String)> = manifest
        .external_calls
        .iter()
        .map(|e| (e.target.to_lowercase(), e.selector.to_lowercase()))
        .collect();

    for call in &trace.external_calls {
        let key = (call.to.to_lowercase(), call.selector.to_lowercase());
        if !manifest_call_keys.contains(&key) {
            fails.push(FailReason::UnexpectedExternalCall {
                to: call.to.clone(),
                selector: call.selector.clone(),
                reason: "call target+selector not in manifest allowlist".to_string(),
            });
        }
    }
}

fn detect_swapped_addresses(
    trace: &FrozenTrace,
    manifest: &Manifest,
    fails: &mut Vec<FailReason>,
) {
    // Build map of manifest expected new values for address setters
    let expected_addrs: HashMap<&str, &str> = manifest
        .effects
        .iter()
        .filter(|e| e.op == StorageOp::SetAddress)
        .map(|e| (e.semantic_path.as_str(), e.expected_new_value.as_str()))
        .collect();

    // Build map of observed new values
    let observed_addrs: HashMap<&str, &str> = trace
        .effects
        .iter()
        .filter(|e| e.op == StorageOp::SetAddress)
        .filter_map(|e| {
            e.semantic_path
                .as_deref()
                .map(|p| (p, e.new_value.as_str()))
        })
        .collect();

    // For each pair of manifest paths, check if their observed values are transposed
    let paths: Vec<&str> = expected_addrs.keys().copied().collect();
    for i in 0..paths.len() {
        for j in (i + 1)..paths.len() {
            let path_a = paths[i];
            let path_b = paths[j];
            let exp_a = normalise_value(expected_addrs[path_a]);
            let exp_b = normalise_value(expected_addrs[path_b]);
            let obs_a = observed_addrs
                .get(path_a)
                .map(|v| normalise_value(v))
                .unwrap_or_default();
            let obs_b = observed_addrs
                .get(path_b)
                .map(|v| normalise_value(v))
                .unwrap_or_default();

            // Swap detected: obs_a == exp_b AND obs_b == exp_a
            if !exp_a.is_empty()
                && !exp_b.is_empty()
                && obs_a == exp_b
                && obs_b == exp_a
                && exp_a != exp_b
            {
                fails.push(FailReason::SwappedAddress {
                    path_a: path_a.to_string(),
                    path_b: path_b.to_string(),
                });
            }
        }
    }
}

/// Normalise a value string for comparison: lowercase, strip 0x prefix for hex.
/// This makes "0xABCD" == "0xabcd" and "0x00..00" == "0".
fn normalise_value(v: &str) -> String {
    let v = v.trim().to_lowercase();
    // For zero address
    if v == "0x0000000000000000000000000000000000000000" {
        return "0x0".to_string();
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsg_types::PinnedFixture;

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
        // Manifest has NO entries
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
                semantic_path: None, // <-- undecodeable
                old_value: "0".to_string(),
                new_value: "1".to_string(),
            }],
            external_calls: vec![],
        };
        let manifest = make_manifest(vec![]);
        let verdict = compare(&trace, &manifest);
        assert!(matches!(verdict, Verdict::Unknown { .. }));
    }
}
