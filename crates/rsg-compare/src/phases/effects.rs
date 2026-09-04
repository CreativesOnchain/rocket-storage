//! Phase 1-3: Observed storage effects scanning, required effect verification,
//! and undeclared write detection.

use std::collections::HashMap;

use rsg_types::{
    FailReason, Manifest, ManifestEffect, ObservedEffect, RequirementLevel, UnknownReason,
};

use crate::normalizer::{is_valid_deletion_target, normalise_value};

/// Group observed effects by semantic path and collect any undecodable keys.
pub fn scan_observed_effects<'a>(
    effects: &'a [ObservedEffect],
    unknown_reasons: &mut Vec<UnknownReason>,
) -> HashMap<String, Vec<&'a ObservedEffect>> {
    let mut observed_counts: HashMap<String, Vec<&'a ObservedEffect>> = HashMap::new();

    for effect in effects {
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

    observed_counts
}

/// Verify that every REQUIRED manifest effect is present and observed with the correct multiplicity and values.
pub fn check_manifest_effects(
    manifest: &Manifest,
    observed_counts: &HashMap<String, Vec<&ObservedEffect>>,
    fail_reasons: &mut Vec<FailReason>,
) {
    for entry in &manifest.effects {
        if entry.requirement != RequirementLevel::Required {
            continue;
        }

        let observed = observed_counts.get(&entry.semantic_path);
        match observed {
            None => {
                fail_reasons.push(FailReason::MissingRequiredEffect {
                    semantic_path: entry.semantic_path.clone(),
                });
            }
            Some(effects) => {
                if effects.len() != entry.multiplicity {
                    fail_reasons.push(FailReason::DuplicateMutation {
                        semantic_path: entry.semantic_path.clone(),
                        expected: entry.multiplicity,
                        observed: effects.len(),
                    });
                }

                for obs in effects {
                    check_effect_against_entry(obs, entry, fail_reasons);
                }
            }
        }
    }
}

/// Check for storage writes observed during execution that have no corresponding entry in the manifest.
pub fn check_undeclared_writes(
    manifest: &Manifest,
    observed_counts: &HashMap<String, Vec<&ObservedEffect>>,
    fail_reasons: &mut Vec<FailReason>,
) {
    let manifest_paths: HashMap<&str, &ManifestEffect> =
        manifest.effects.iter().map(|e| (e.semantic_path.as_str(), e)).collect();

    for (path, effects) in observed_counts {
        if !manifest_paths.contains_key(path.as_str()) {
            let first = effects[0];
            fail_reasons.push(FailReason::UndeclaredWrite {
                raw_key: first.raw_key.clone(),
                op: format!("{:?}", first.op),
                new_value: first.new_value.clone(),
            });
        }
    }
}

/// Validate an observed effect against a declared manifest entry for type drift,
/// old/new value divergence, and omitted deletion.
pub fn check_effect_against_entry(
    obs: &ObservedEffect,
    entry: &ManifestEffect,
    fails: &mut Vec<FailReason>,
) {
    // Type check
    if obs.op != entry.op {
        fails.push(FailReason::TypeDrift {
            raw_key: obs.raw_key.clone(),
            expected_op: format!("{:?}", entry.op),
            observed_op: format!("{:?}", obs.op),
        });
        return; // Skip value check if the operation type does not match
    }

    // Old value check (skip if manifest allows "any")
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

    // New value check (skip if manifest allows "any")
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
    if entry.op.is_delete() && !is_valid_deletion_target(&obs.new_value) {
        fails.push(FailReason::OmittedDeletion { semantic_path: entry.semantic_path.clone() });
    }
}
