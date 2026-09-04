//! Proof bundle assembly and filesystem writing.

use std::{fs::write, path::Path};

use anyhow::Result;
use indexmap::IndexMap;
use rsg_types::{
    AttestationBundle, AttestationHashes, FrozenTrace, Manifest, Verdict,
};

use crate::{
    canonical::canonical_json,
    hash::sha256_hex,
    markdown::render_markdown,
};

/// Build and write the complete proof bundle to `output_dir`.
///
/// Writes:
/// - `attestation.json`
/// - `attestation.md`
/// - `observed-trace.json`
/// - `manifest.lock`
/// - `review-record.json` (copy of source or placeholder)
pub fn write_bundle(
    trace: &FrozenTrace,
    manifest: &Manifest,
    manifest_raw: &str,
    review_record_path: Option<&Path>,
    verdict: &Verdict,
    output_dir: &Path,
) -> Result<()> {
    std::fs::create_dir_all(output_dir)?;

    let trace_json = canonical_json(trace)?;
    let (hashes, review_record_content) = compute_bundle_hashes(
        trace,
        &trace_json,
        manifest_raw,
        review_record_path,
    )?;

    let effect_counts = count_effects_by_op(trace);
    let reasons = collect_verdict_reasons(verdict);

    let bundle = AttestationBundle {
        version: "1".to_string(),
        generated_at: Some(chrono::Utc::now().to_rfc3339()),
        hashes: hashes.clone(),
        pinned: trace.pinned.clone(),
        verdict: verdict.clone(),
        reasons,
        effect_counts,
        observation_boundary: AttestationBundle::OBSERVATION_BOUNDARY.to_string(),
        disclaimer: AttestationBundle::DISCLAIMER.to_string(),
    };

    write_bundle_files(
        output_dir,
        &bundle,
        trace,
        manifest,
        &trace_json,
        manifest_raw,
        &review_record_content,
    )?;

    eprintln!("[rsg] Proof bundle written to {}", output_dir.display());
    eprintln!("[rsg] Observed-trace SHA-256: {}", hashes.observed_trace_sha256);
    eprintln!("[rsg] Manifest SHA-256: {}", hashes.manifest_sha256);

    Ok(())
}

fn compute_bundle_hashes(
    trace: &FrozenTrace,
    trace_json: &str,
    manifest_raw: &str,
    review_record_path: Option<&Path>,
) -> Result<(AttestationHashes, String)> {
    let observed_trace_sha256 = sha256_hex(trace_json.as_bytes());
    let manifest_sha256 = sha256_hex(manifest_raw.as_bytes());

    let (review_record_sha256, review_record_content) = if let Some(rr_path) = review_record_path {
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

    Ok((hashes, review_record_content))
}

fn count_effects_by_op(trace: &FrozenTrace) -> IndexMap<String, usize> {
    let mut effect_counts: IndexMap<String, usize> = IndexMap::new();
    for eff in &trace.effects {
        let key = format!("{:?}", eff.op);
        *effect_counts.entry(key).or_insert(0) += 1;
    }
    effect_counts
}

fn collect_verdict_reasons(verdict: &Verdict) -> Vec<serde_json::Value> {
    match verdict {
        Verdict::Pass => vec![],
        Verdict::Fail { reasons } => reasons
            .iter()
            .map(|r| serde_json::to_value(r).unwrap())
            .collect(),
        Verdict::Unknown { reasons } => reasons
            .iter()
            .map(|r| serde_json::to_value(r).unwrap())
            .collect(),
    }
}

fn write_bundle_files(
    output_dir: &Path,
    bundle: &AttestationBundle,
    trace: &FrozenTrace,
    manifest: &Manifest,
    trace_json: &str,
    manifest_raw: &str,
    review_record_content: &str,
) -> Result<()> {
    // attestation.json
    let attest_json = serde_json::to_string_pretty(bundle)?;
    write(output_dir.join("attestation.json"), &attest_json)?;

    // attestation.md
    let attest_md = render_markdown(bundle, trace, manifest);
    write(output_dir.join("attestation.md"), &attest_md)?;

    // observed-trace.json (normalized, no wall-clock timestamps)
    write(output_dir.join("observed-trace.json"), trace_json)?;

    // manifest.lock (snapshot of the manifest used)
    write(output_dir.join("manifest.lock"), manifest_raw)?;

    // review-record.json
    write(output_dir.join("review-record.json"), review_record_content)?;

    Ok(())
}
