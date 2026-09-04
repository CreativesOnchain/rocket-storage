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
