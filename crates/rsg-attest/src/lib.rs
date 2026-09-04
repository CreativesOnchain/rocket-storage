//! Proof bundle generator.
//!
//! Produces a deterministic JSON attestation and a human-readable Markdown
//! report, binding the verdict to input hashes.

use std::fs::write;
use anyhow::Result;
use indexmap::IndexMap;
use rsg_types::{
    AttestationBundle, AttestationHashes, FrozenTrace, Manifest, Verdict,
};
use sha2::{Digest, Sha256};
use std::path::Path;

mod markdown;
pub use markdown::render_markdown;


/// Build and write the complete proof bundle to `output_dir`.
///
/// Writes:
/// - `attestation.json`
/// - `attestation.md`
/// - `observed-trace.json`
/// - `manifest.lock`
/// - `review-record.json` (copy of source)
pub fn write_bundle(
    trace: &FrozenTrace,
    manifest: &Manifest,
    manifest_raw: &str,
    review_record_path: Option<&Path>,
    verdict: &Verdict,
    output_dir: &Path,
) -> Result<()> {
    std::fs::create_dir_all(output_dir)?;

    // ── Compute hashes ────────────────────────────────────────────────────────
    let trace_json = canonical_json(trace)?;
    let observed_trace_sha256 = sha256_hex(trace_json.as_bytes());
    let manifest_sha256 = sha256_hex(manifest_raw.as_bytes());

    let (review_record_sha256, review_record_content) =
        if let Some(rr_path) = review_record_path {
            let content = std::fs::read_to_string(rr_path)?;
            let hash = sha256_hex(content.as_bytes());
            (hash, content)
        } else {
            let placeholder = serde_json::json!({
                "note": "No review-record.json provided. Not yet peer-reviewed.",
            })
            .to_string();
            let hash = sha256_hex(placeholder.as_bytes());
            (hash, placeholder)
        };

    let hashes = AttestationHashes {
        observed_trace_sha256,
        manifest_sha256,
        review_record_sha256,
        upgrade_tx: trace.pinned.upgrade_tx.clone(),
        source_commit: trace.pinned.source_commit.clone(),
        tool_version: format!("rsg/{}", env!("CARGO_PKG_VERSION")),
    };

    // ── Count effects by op ───────────────────────────────────────────────────
    let mut effect_counts: IndexMap<String, usize> = IndexMap::new();
    for eff in &trace.effects {
        let key = format!("{:?}", eff.op);
        *effect_counts.entry(key).or_insert(0) += 1;
    }

    // ── Build reasons list ────────────────────────────────────────────────────
    let reasons: Vec<serde_json::Value> = match verdict {
        Verdict::Pass => vec![],
        Verdict::Fail { reasons } => reasons
            .iter()
            .map(|r| serde_json::to_value(r).unwrap())
            .collect(),
        Verdict::Unknown { reasons } => reasons
            .iter()
            .map(|r| serde_json::to_value(r).unwrap())
            .collect(),
    };

    // ── Assemble attestation bundle ───────────────────────────────────────────
    let bundle = AttestationBundle {
        version: "1".to_string(),
        generated_at: Some(chrono::Utc::now().to_rfc3339()),
        hashes: hashes.clone(),
        pinned: trace.pinned.clone(),
        verdict: verdict.clone(),
        reasons,
        effect_counts: effect_counts.clone(),
        observation_boundary: AttestationBundle::OBSERVATION_BOUNDARY.to_string(),
        disclaimer: AttestationBundle::DISCLAIMER.to_string(),
    };

    // ── Write files ───────────────────────────────────────────────────────────

    // attestation.json
    let attest_json = serde_json::to_string_pretty(&bundle)?;
    write(output_dir.join("attestation.json"), &attest_json)?;

    // attestation.md
    let attest_md = render_markdown(&bundle, trace, manifest);
    write(output_dir.join("attestation.md"), &attest_md)?;

    // observed-trace.json  (normalized, no wall-clock timestamps)
    write(output_dir.join("observed-trace.json"), &trace_json)?;

    // manifest.lock  (snapshot of the manifest used)
    write(output_dir.join("manifest.lock"), manifest_raw)?;

    // review-record.json
    write(
        output_dir.join("review-record.json"),
        &review_record_content,
    )?;

    eprintln!("[rsg] Proof bundle written to {}", output_dir.display());
    eprintln!("[rsg] Observed-trace SHA-256: {}", hashes.observed_trace_sha256);
    eprintln!("[rsg] Manifest SHA-256: {}", hashes.manifest_sha256);

    Ok(())
}

// ─── Canonical JSON ───────────────────────────────────────────────────────────

/// Serialize to canonical JSON: sorted keys, no wall-clock timestamps.
/// The `generated_at` field is intentionally excluded from the trace hash.
fn canonical_json(trace: &FrozenTrace) -> Result<String> {
    // We serialize FrozenTrace directly — it contains no timestamps.
    // Use serde_json's default serialization (fields in struct order),
    // then re-parse and re-serialize with sorted keys for stability.
    let value = serde_json::to_value(trace)?;
    let sorted = sort_json_value(value);
    Ok(serde_json::to_string(&sorted)?)
}

/// Recursively sort JSON object keys for canonical form.
fn sort_json_value(v: serde_json::Value) -> serde_json::Value {
    match v {
        serde_json::Value::Object(map) => {
            let mut sorted: serde_json::Map<String, serde_json::Value> =
                serde_json::Map::new();
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            for k in keys {
                let val = map[&k].clone();
                sorted.insert(k, sort_json_value(val));
            }
            serde_json::Value::Object(sorted)
        }
        serde_json::Value::Array(arr) => {
            serde_json::Value::Array(arr.into_iter().map(sort_json_value).collect())
        }
        other => other,
    }
}

// ─── SHA-256 ──────────────────────────────────────────────────────────────────

fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Return the SHA-256 hex of arbitrary content (public helper for CLI).
pub fn hash_file(path: &Path) -> Result<String> {
    let data = std::fs::read(path)?;
    Ok(sha256_hex(&data))
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

    #[test]
    fn sha256_is_stable() {
        let data = b"test input";
        let h1 = sha256_hex(data);
        let h2 = sha256_hex(data);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }
}
