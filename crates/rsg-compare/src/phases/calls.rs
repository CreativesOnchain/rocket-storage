//! Phase 4: External call allowlist and multiplicity verification.

use std::collections::{HashMap, HashSet};

use rsg_types::{FailReason, FrozenTrace, Manifest, ManifestExternalCall, ObservedExternalCall};

/// Check observed external calls against manifest allowlist and multiplicity requirements.
pub fn check_external_calls(trace: &FrozenTrace, manifest: &Manifest, fails: &mut Vec<FailReason>) {
    let observed = count_observed_calls(&trace.external_calls);
    verify_manifest_calls(&manifest.external_calls, &observed, fails);
    detect_unauthorized_calls(&trace.external_calls, &manifest.external_calls, fails);
}

/// Aggregate observed external calls by canonical `(target, selector)` tuple.
pub fn count_observed_calls(calls: &[ObservedExternalCall]) -> HashMap<(String, String), usize> {
    let mut observed: HashMap<(String, String), usize> = HashMap::new();
    for call in calls {
        let key = call_key(&call.to, &call.selector);
        *observed.entry(key).or_default() += 1;
    }
    observed
}

/// Verify that each expected external call in the manifest is observed with the declared multiplicity.
pub fn verify_manifest_calls(
    expected_calls: &[ManifestExternalCall],
    observed_counts: &HashMap<(String, String), usize>,
    fails: &mut Vec<FailReason>,
) {
    for expected in expected_calls {
        let key = call_key(&expected.target, &expected.selector);
        let count = observed_counts.get(&key).copied().unwrap_or(0);

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
}

/// Detect any observed external call that does not appear in the manifest allowlist.
pub fn detect_unauthorized_calls(
    observed_calls: &[ObservedExternalCall],
    expected_calls: &[ManifestExternalCall],
    fails: &mut Vec<FailReason>,
) {
    let manifest_call_keys: HashSet<(String, String)> = expected_calls
        .iter()
        .map(|e| call_key(&e.target, &e.selector))
        .collect();

    for call in observed_calls {
        let key = call_key(&call.to, &call.selector);
        if !manifest_call_keys.contains(&key) {
            fails.push(FailReason::UnexpectedExternalCall {
                to: call.to.clone(),
                selector: call.selector.clone(),
                reason: "call target+selector not in manifest allowlist".to_string(),
            });
        }
    }
}

/// Normalize an address target and 4-byte selector pair into lowercase canonical form.
#[inline]
pub fn call_key(target: &str, selector: &str) -> (String, String) {
    (target.to_lowercase(), selector.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_observed(to: &str, selector: &str) -> ObservedExternalCall {
        ObservedExternalCall {
            call_index: 0,
            from: "0xupgrade".to_string(),
            to: to.to_string(),
            selector: selector.to_string(),
            eth_value: "0".to_string(),
            success: true,
        }
    }

    fn make_manifest_entry(to: &str, selector: &str, multiplicity: usize) -> ManifestExternalCall {
        ManifestExternalCall {
            target: to.to_string(),
            selector: selector.to_string(),
            eth_value: "0".to_string(),
            multiplicity,
            rationale: "test".to_string(),
        }
    }

    #[test]
    fn test_count_observed_calls() {
        let calls = vec![
            make_observed("0xABCD", "0x1234"),
            make_observed("0xabcd", "0x1234"),
            make_observed("0xbeef", "0x5678"),
        ];

        let counts = count_observed_calls(&calls);
        assert_eq!(
            counts.get(&("0xabcd".to_string(), "0x1234".to_string())),
            Some(&2)
        );
        assert_eq!(
            counts.get(&("0xbeef".to_string(), "0x5678".to_string())),
            Some(&1)
        );
    }

    #[test]
    fn test_verify_manifest_calls_mismatch() {
        let expected = vec![make_manifest_entry("0xvault", "0x1111", 2)];
        let mut observed = HashMap::new();
        observed.insert(("0xvault".to_string(), "0x1111".to_string()), 1);

        let mut fails = Vec::new();
        verify_manifest_calls(&expected, &observed, &mut fails);
        assert_eq!(fails.len(), 1);
        assert!(matches!(
            fails[0],
            FailReason::UnexpectedExternalCall { .. }
        ));
    }

    #[test]
    fn test_detect_unauthorized_calls() {
        let observed = vec![
            make_observed("0xauthorized", "0x1111"),
            make_observed("0xunauthorized", "0x2222"),
        ];
        let expected = vec![make_manifest_entry("0xauthorized", "0x1111", 1)];

        let mut fails = Vec::new();
        detect_unauthorized_calls(&observed, &expected, &mut fails);
        assert_eq!(fails.len(), 1);
        match &fails[0] {
            FailReason::UnexpectedExternalCall { to, selector, .. } => {
                assert_eq!(to, "0xunauthorized");
                assert_eq!(selector, "0x2222");
            }
            _ => panic!("expected UnexpectedExternalCall"),
        }
    }
}
