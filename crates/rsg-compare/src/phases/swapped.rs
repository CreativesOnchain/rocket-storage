//! Phase 5: Swapped address detection.
//!
//! Detects critical security vulnerabilities where two address slots are mistakenly
//! transposed (e.g. `contract.address[rocketVault]` <-> `contract.address[rocketTokenRETH]`).

use std::collections::HashMap;

use rsg_types::{FailReason, FrozenTrace, Manifest, StorageOp};

use crate::normalizer::normalise_value;

/// Detect if any pair of address setters in the manifest have had their values transposed in the observed trace.
pub fn detect_swapped_addresses(
    trace: &FrozenTrace,
    manifest: &Manifest,
    fails: &mut Vec<FailReason>,
) {
    let expected_addrs = extract_expected_address_map(manifest);
    let observed_addrs = extract_observed_address_map(trace);

    let swapped_pairs = find_swapped_address_pairs(&expected_addrs, &observed_addrs);
    for (path_a, path_b) in swapped_pairs {
        fails.push(FailReason::SwappedAddress {
            path_a: path_a.to_string(),
            path_b: path_b.to_string(),
        });
    }
}

/// Extract a map of `semantic_path -> expected_new_value` for all `SetAddress` mutations in the manifest.
pub fn extract_expected_address_map(manifest: &Manifest) -> HashMap<&str, &str> {
    manifest
        .effects
        .iter()
        .filter(|e| e.op == StorageOp::SetAddress)
        .map(|e| (e.semantic_path.as_str(), e.expected_new_value.as_str()))
        .collect()
}

/// Extract a map of `semantic_path -> observed_new_value` for all `SetAddress` mutations in the observed trace.
pub fn extract_observed_address_map(trace: &FrozenTrace) -> HashMap<&str, &str> {
    trace
        .effects
        .iter()
        .filter(|e| e.op == StorageOp::SetAddress)
        .filter_map(|e| {
            e.semantic_path
                .as_deref()
                .map(|p| (p, e.new_value.as_str()))
        })
        .collect()
}

/// Check if two address paths have their expected and observed values transposed.
pub fn is_transposed_pair(exp_a: &str, exp_b: &str, obs_a: &str, obs_b: &str) -> bool {
    let exp_a_norm = normalise_value(exp_a);
    let exp_b_norm = normalise_value(exp_b);
    let obs_a_norm = normalise_value(obs_a);
    let obs_b_norm = normalise_value(obs_b);

    !exp_a_norm.is_empty()
        && !exp_b_norm.is_empty()
        && obs_a_norm == exp_b_norm
        && obs_b_norm == exp_a_norm
        && exp_a_norm != exp_b_norm
}

/// Find all pairs of paths in `expected_addrs` whose observed values in `observed_addrs` are transposed.
///
/// Returns a deterministic list of `(path_a, path_b)` tuples.
pub fn find_swapped_address_pairs<'a>(
    expected_addrs: &HashMap<&'a str, &'a str>,
    observed_addrs: &HashMap<&str, &str>,
) -> Vec<(&'a str, &'a str)> {
    let mut paths: Vec<&'a str> = expected_addrs.keys().copied().collect();
    paths.sort_unstable();

    let mut swapped = Vec::new();
    for i in 0..paths.len() {
        for j in (i + 1)..paths.len() {
            let path_a = paths[i];
            let path_b = paths[j];
            let exp_a = expected_addrs[path_a];
            let exp_b = expected_addrs[path_b];
            let obs_a = observed_addrs.get(path_a).copied().unwrap_or_default();
            let obs_b = observed_addrs.get(path_b).copied().unwrap_or_default();

            if is_transposed_pair(exp_a, exp_b, obs_a, obs_b) {
                swapped.push((path_a, path_b));
            }
        }
    }
    swapped
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsg_types::{ManifestEffect, ObservedEffect, RequirementLevel};

    #[test]
    fn test_is_transposed_pair_true_swap() {
        assert!(is_transposed_pair(
            "0x1111111111111111111111111111111111111111",
            "0x2222222222222222222222222222222222222222",
            "0x2222222222222222222222222222222222222222",
            "0x1111111111111111111111111111111111111111",
        ));
    }

    #[test]
    fn test_is_transposed_pair_identical_values_not_swap() {
        assert!(!is_transposed_pair("0x1111", "0x1111", "0x1111", "0x1111"));
    }

    #[test]
    fn test_is_transposed_pair_no_transposition() {
        assert!(!is_transposed_pair("0x1111", "0x2222", "0x1111", "0x2222"));
    }

    #[test]
    fn test_find_swapped_address_pairs() {
        let mut expected = HashMap::new();
        expected.insert("contract.address[vault]", "0xAAAA");
        expected.insert("contract.address[token]", "0xBBBB");
        expected.insert("contract.address[other]", "0xCCCC");

        let mut observed = HashMap::new();
        // Swap vault and token
        observed.insert("contract.address[vault]", "0xbbbb");
        observed.insert("contract.address[token]", "0xaaaa");
        observed.insert("contract.address[other]", "0xcccc");

        let swapped = find_swapped_address_pairs(&expected, &observed);
        assert_eq!(swapped.len(), 1);
        assert_eq!(
            swapped[0],
            ("contract.address[token]", "contract.address[vault]")
        );
    }

    #[test]
    fn test_detect_swapped_addresses_end_to_end() {
        let manifest = Manifest {
            version: "1".to_string(),
            fixture: rsg_types::PinnedFixture::default(),
            effects: vec![
                ManifestEffect {
                    semantic_path: "contract.a".to_string(),
                    raw_key: None,
                    op: StorageOp::SetAddress,
                    expected_old_value: "any".to_string(),
                    expected_new_value: "0x1111".to_string(),
                    requirement: RequirementLevel::Required,
                    multiplicity: 1,
                    source_anchor: "test".to_string(),
                    rationale: "test".to_string(),
                },
                ManifestEffect {
                    semantic_path: "contract.b".to_string(),
                    raw_key: None,
                    op: StorageOp::SetAddress,
                    expected_old_value: "any".to_string(),
                    expected_new_value: "0x2222".to_string(),
                    requirement: RequirementLevel::Required,
                    multiplicity: 1,
                    source_anchor: "test".to_string(),
                    rationale: "test".to_string(),
                },
            ],
            external_calls: vec![],
        };

        let trace = FrozenTrace {
            pinned: rsg_types::PinnedFixture::default(),
            effects: vec![
                ObservedEffect {
                    call_index: 0,
                    caller: "0xcaller".to_string(),
                    op: StorageOp::SetAddress,
                    raw_key: "0xkey1".to_string(),
                    semantic_path: Some("contract.a".to_string()),
                    old_value: "0x0".to_string(),
                    new_value: "0x2222".to_string(),
                },
                ObservedEffect {
                    call_index: 1,
                    caller: "0xcaller".to_string(),
                    op: StorageOp::SetAddress,
                    raw_key: "0xkey2".to_string(),
                    semantic_path: Some("contract.b".to_string()),
                    old_value: "0x0".to_string(),
                    new_value: "0x1111".to_string(),
                },
            ],
            external_calls: vec![],
        };

        let mut fails = Vec::new();
        detect_swapped_addresses(&trace, &manifest, &mut fails);
        assert_eq!(fails.len(), 1);
        assert!(matches!(fails[0], FailReason::SwappedAddress { .. }));
    }
}
