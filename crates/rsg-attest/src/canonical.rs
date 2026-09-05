//! Deterministic, key-sorted JSON canonicalization.

use anyhow::Result;
use rsg_types::FrozenTrace;
use serde_json::Value;

/// Serialize to canonical JSON: recursively sorted keys, no timestamps.
///
/// This provides a stable hash input regardless of key ordering in memory.
pub fn canonical_json(trace: &FrozenTrace) -> Result<String> {
    let value = serde_json::to_value(trace)?;
    let sorted = sort_json_value(value);
    Ok(serde_json::to_string(&sorted)?)
}

/// Recursively sort JSON object keys into lexicographical order.
pub fn sort_json_value(v: Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut sorted = serde_json::Map::new();
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            for k in keys {
                let val = map[&k].clone();
                sorted.insert(k, sort_json_value(val));
            }
            Value::Object(sorted)
        }
        Value::Array(arr) => Value::Array(arr.into_iter().map(sort_json_value).collect()),
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsg_types::PinnedFixture;

    #[test]
    fn canonical_json_is_stable() {
        let trace = FrozenTrace {
            pinned: PinnedFixture::default(),
            effects: vec![],
            external_calls: vec![],
        };
        let a = canonical_json(&trace).unwrap();
        let b = canonical_json(&trace).unwrap();
        assert_eq!(a, b);
    }
}
