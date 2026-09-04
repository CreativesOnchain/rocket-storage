//! Phase 4: External call allowlist and multiplicity verification.

use std::collections::{HashMap, HashSet};

use rsg_types::{FailReason, FrozenTrace, Manifest};

/// Check observed external calls against manifest allowlist and multiplicity requirements.
pub fn check_external_calls(trace: &FrozenTrace, manifest: &Manifest, fails: &mut Vec<FailReason>) {
    // Count observed external calls by (to, selector)
    let mut observed: HashMap<(String, String), usize> = HashMap::new();
    for call in &trace.external_calls {
        let key = (call.to.to_lowercase(), call.selector.to_lowercase());
        *observed.entry(key).or_default() += 1;
    }

    // Check each manifest external call entry
    for expected in &manifest.external_calls {
        let key = (expected.target.to_lowercase(), expected.selector.to_lowercase());
        let count = observed.get(&key).copied().unwrap_or(0);

        if count != expected.multiplicity {
            fails.push(FailReason::UnexpectedExternalCall {
                to: expected.target.clone(),
                selector: expected.selector.clone(),
                reason: format!(
                    "expected {} call(s) to {}:{}, observed {}",
                    expected.multiplicity, expected.target, expected.selector, count
                ),
            });
        }
    }

    // Check for any observed call not present in the manifest allowlist
    let manifest_call_keys: HashSet<(String, String)> = manifest
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
