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
                validate_effect_multiplicity(entry, effects.len(), fail_reasons);

                for obs in effects {
                    check_effect_against_entry(obs, entry, fail_reasons);
                }
            }
        }
    }
}

/// Validate that the observed multiplicity matches the declared requirement in the manifest.
pub fn validate_effect_multiplicity(
    entry: &ManifestEffect,
    observed_count: usize,
    fail_reasons: &mut Vec<FailReason>,
) {
    if observed_count != entry.multiplicity {
        fail_reasons.push(FailReason::DuplicateMutation {
            semantic_path: entry.semantic_path.clone(),
            expected: entry.multiplicity,
            observed: observed_count,
        });
    }
}

/// Check for storage writes observed during execution that have no corresponding entry in the manifest.
pub fn check_undeclared_writes(
    manifest: &Manifest,
    observed_counts: &HashMap<String, Vec<&ObservedEffect>>,
    fail_reasons: &mut Vec<FailReason>,
) {
    let manifest_paths: HashMap<&str, &ManifestEffect> = manifest
        .effects
        .iter()
        .map(|e| (e.semantic_path.as_str(), e))
        .collect();

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
    if let Err(type_err) = validate_operation_type(obs, entry) {
        fails.push(type_err);
        return; // Skip value check if the operation type does not match
    }

    if let Err(old_err) = validate_old_value(obs, entry) {
        fails.push(old_err);
    }

    if let Err(new_err) = validate_new_value(obs, entry) {
        fails.push(new_err);
    }

    if let Err(del_err) = validate_deletion_target(obs, entry) {
        fails.push(del_err);
    }
}

/// Validate that the observed operation matches the manifest entry's expected storage operation.
pub fn validate_operation_type(
    obs: &ObservedEffect,
    entry: &ManifestEffect,
) -> Result<(), FailReason> {
    if obs.op != entry.op {
        Err(FailReason::TypeDrift {
            raw_key: obs.raw_key.clone(),
            expected_op: format!("{:?}", entry.op),
            observed_op: format!("{:?}", obs.op),
        })
    } else {
        Ok(())
    }
}

/// Validate that the observed old value matches the manifest entry, unless "any" is permitted.
pub fn validate_old_value(obs: &ObservedEffect, entry: &ManifestEffect) -> Result<(), FailReason> {
    if entry.expected_old_value != "any"
        && normalise_value(&obs.old_value) != normalise_value(&entry.expected_old_value)
    {
        Err(FailReason::WrongValue {
            semantic_path: entry.semantic_path.clone(),
            field: "old_value".to_string(),
            expected: entry.expected_old_value.clone(),
            observed: obs.old_value.clone(),
        })
    } else {
        Ok(())
    }
}

/// Validate that the observed new value matches the manifest entry, unless "any" is permitted.
pub fn validate_new_value(obs: &ObservedEffect, entry: &ManifestEffect) -> Result<(), FailReason> {
    if entry.expected_new_value != "any"
        && normalise_value(&obs.new_value) != normalise_value(&entry.expected_new_value)
    {
        Err(FailReason::WrongValue {
            semantic_path: entry.semantic_path.clone(),
            field: "new_value".to_string(),
            expected: entry.expected_new_value.clone(),
            observed: obs.new_value.clone(),
        })
    } else {
        Ok(())
    }
}

/// Validate that a deletion operation successfully targets an empty or zeroed slot representation.
pub fn validate_deletion_target(
    obs: &ObservedEffect,
    entry: &ManifestEffect,
) -> Result<(), FailReason> {
    if entry.op.is_delete() && !is_valid_deletion_target(&obs.new_value) {
        Err(FailReason::OmittedDeletion {
            semantic_path: entry.semantic_path.clone(),
        })
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsg_types::StorageOp;

    fn make_obs(path: Option<&str>, op: StorageOp, old_val: &str, new_val: &str) -> ObservedEffect {
        ObservedEffect {
            call_index: 0,
            caller: "0xcaller".to_string(),
            semantic_path: path.map(String::from),
            raw_key: "0xkey".to_string(),
            op,
            old_value: old_val.to_string(),
            new_value: new_val.to_string(),
        }
    }

    fn make_manifest_entry(
        path: &str,
        op: StorageOp,
        old_val: &str,
        new_val: &str,
        req: RequirementLevel,
        multiplicity: usize,
    ) -> ManifestEffect {
        ManifestEffect {
            semantic_path: path.to_string(),
            raw_key: None,
            op,
            expected_old_value: old_val.to_string(),
            expected_new_value: new_val.to_string(),
            requirement: req,
            multiplicity,
            source_anchor: "contracts/Test.sol:10".to_string(),
            rationale: "test".to_string(),
        }
    }

    #[test]
    fn test_scan_observed_effects_grouping_and_unknown() {
        let effects = vec![
            make_obs(
                Some("contract.address[test]"),
                StorageOp::SetAddress,
                "0x0",
                "0x1",
            ),
            make_obs(None, StorageOp::SetUint, "0", "1"),
        ];

        let mut unknowns = Vec::new();
        let grouped = scan_observed_effects(&effects, &mut unknowns);

        assert_eq!(unknowns.len(), 1);
        assert!(matches!(unknowns[0], UnknownReason::UndecodeableKey { .. }));
        assert_eq!(
            grouped.get("contract.address[test]").map(|v| v.len()),
            Some(1)
        );
    }

    #[test]
    fn test_validate_effect_multiplicity() {
        let entry = make_manifest_entry(
            "test.path",
            StorageOp::SetUint,
            "0",
            "1",
            RequirementLevel::Required,
            1,
        );
        let mut fails = Vec::new();

        validate_effect_multiplicity(&entry, 2, &mut fails);
        assert_eq!(fails.len(), 1);
        assert!(matches!(
            fails[0],
            FailReason::DuplicateMutation {
                expected: 1,
                observed: 2,
                ..
            }
        ));

        fails.clear();
        validate_effect_multiplicity(&entry, 1, &mut fails);
        assert!(fails.is_empty());
    }

    #[test]
    fn test_validate_operation_type_mismatch() {
        let obs = make_obs(Some("test.path"), StorageOp::SetUint, "0", "1");
        let entry = make_manifest_entry(
            "test.path",
            StorageOp::SetAddress,
            "0",
            "1",
            RequirementLevel::Required,
            1,
        );

        let res = validate_operation_type(&obs, &entry);
        assert!(matches!(res, Err(FailReason::TypeDrift { .. })));
    }

    #[test]
    fn test_validate_old_and_new_value_checks() {
        let obs = make_obs(Some("test.path"), StorageOp::SetUint, " 0xABCD ", "42");
        let entry_exact = make_manifest_entry(
            "test.path",
            StorageOp::SetUint,
            "0xabcd",
            "42",
            RequirementLevel::Required,
            1,
        );
        assert!(validate_old_value(&obs, &entry_exact).is_ok());
        assert!(validate_new_value(&obs, &entry_exact).is_ok());

        let entry_wildcard = make_manifest_entry(
            "test.path",
            StorageOp::SetUint,
            "any",
            "any",
            RequirementLevel::Required,
            1,
        );
        assert!(validate_old_value(&obs, &entry_wildcard).is_ok());
        assert!(validate_new_value(&obs, &entry_wildcard).is_ok());

        let entry_mismatch = make_manifest_entry(
            "test.path",
            StorageOp::SetUint,
            "0x99",
            "0x88",
            RequirementLevel::Required,
            1,
        );
        assert!(matches!(
            validate_old_value(&obs, &entry_mismatch),
            Err(FailReason::WrongValue { .. })
        ));
        assert!(matches!(
            validate_new_value(&obs, &entry_mismatch),
            Err(FailReason::WrongValue { .. })
        ));
    }

    #[test]
    fn test_validate_deletion_target() {
        let obs_valid = make_obs(
            Some("test.path"),
            StorageOp::DeleteAddress,
            "0x1234",
            "0x0000000000000000000000000000000000000000",
        );
        let entry_del = make_manifest_entry(
            "test.path",
            StorageOp::DeleteAddress,
            "any",
            "0x0",
            RequirementLevel::Required,
            1,
        );
        assert!(validate_deletion_target(&obs_valid, &entry_del).is_ok());

        let obs_invalid = make_obs(
            Some("test.path"),
            StorageOp::DeleteAddress,
            "0x1234",
            "0x9999",
        );
        assert!(matches!(
            validate_deletion_target(&obs_invalid, &entry_del),
            Err(FailReason::OmittedDeletion { .. })
        ));
    }

    #[test]
    fn test_check_manifest_effects_missing_required() {
        let manifest = Manifest {
            version: "1".to_string(),
            fixture: rsg_types::PinnedFixture::default(),
            effects: vec![make_manifest_entry(
                "required.missing",
                StorageOp::SetUint,
                "0",
                "1",
                RequirementLevel::Required,
                1,
            )],
            external_calls: vec![],
        };

        let observed_counts = HashMap::new();
        let mut fails = Vec::new();
        check_manifest_effects(&manifest, &observed_counts, &mut fails);

        assert_eq!(fails.len(), 1);
        assert!(matches!(fails[0], FailReason::MissingRequiredEffect { .. }));
    }

    #[test]
    fn test_check_undeclared_writes() {
        let manifest = Manifest {
            version: "1".to_string(),
            fixture: rsg_types::PinnedFixture::default(),
            effects: vec![],
            external_calls: vec![],
        };

        let obs = make_obs(Some("unexpected.write"), StorageOp::SetUint, "0", "1");
        let mut observed_counts = HashMap::new();
        observed_counts.insert("unexpected.write".to_string(), vec![&obs]);

        let mut fails = Vec::new();
        check_undeclared_writes(&manifest, &observed_counts, &mut fails);

        assert_eq!(fails.len(), 1);
        assert!(matches!(fails[0], FailReason::UndeclaredWrite { .. }));
    }
}
