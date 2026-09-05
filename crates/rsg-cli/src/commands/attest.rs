//! Handler for `rsg attest`.

use std::path::Path;

use anyhow::{Context, Result};
use rsg_attest::write_bundle;
use rsg_capture::{capture_live, load_frozen_trace};
use rsg_compare::compare;
use rsg_types::{FrozenTrace, Manifest, Verdict};

/// Execute attestation comparison against manifest and write out proof bundle.
pub async fn execute(
    fixture: Option<&Path>,
    rpc_url: Option<&str>,
    manifest_path: &Path,
    review_record: Option<&Path>,
    output_dir: &Path,
) -> Result<i32> {
    let trace = load_or_capture_trace(fixture, rpc_url).await?;
    let (manifest, manifest_raw) = load_manifest(manifest_path)?;

    eprintln!("[rsg] Running comparator…");
    let verdict = compare(&trace, &manifest);

    write_bundle(
        &trace,
        &manifest,
        &manifest_raw,
        review_record,
        &verdict,
        output_dir,
    )?;

    let code = verdict.exit_code();
    print_verdict_summary(&verdict, trace.effects.len(), output_dir, code);

    Ok(code)
}

async fn load_or_capture_trace(
    fixture: Option<&Path>,
    rpc_url: Option<&str>,
) -> Result<FrozenTrace> {
    match (fixture, rpc_url) {
        (Some(path), _) => {
            eprintln!("[rsg] Loading frozen trace from {}…", path.display());
            load_frozen_trace(path)
        }
        (_, Some(url)) => {
            eprintln!("[rsg] Capturing live trace…");
            capture_live(url).await
        }
        _ => anyhow::bail!("must specify either --fixture or --rpc-url"),
    }
}

fn load_manifest(manifest_path: &Path) -> Result<(Manifest, String)> {
    eprintln!("[rsg] Loading manifest from {}…", manifest_path.display());
    let manifest_raw = std::fs::read_to_string(manifest_path)
        .with_context(|| format!("cannot read manifest: {}", manifest_path.display()))?;
    let manifest: Manifest = serde_yaml::from_str(&manifest_raw).context("manifest parse error")?;
    Ok((manifest, manifest_raw))
}

fn print_verdict_summary(
    verdict: &Verdict,
    effects_count: usize,
    output_dir: &Path,
    exit_code: i32,
) {
    eprintln!();
    match verdict {
        Verdict::Pass => {
            println!("✅  PASS");
            println!("    All {effects_count} effects match the manifest.");
        }
        Verdict::Fail { reasons } => {
            println!("❌  FAIL ({} reason(s))", reasons.len());
            for r in reasons {
                println!("    • {}", serde_json::to_string(r).unwrap_or_default());
            }
        }
        Verdict::Unknown { reasons } => {
            println!("⚠️  UNKNOWN ({} reason(s))", reasons.len());
            for r in reasons {
                println!("    • {}", serde_json::to_string(r).unwrap_or_default());
            }
        }
    }
    println!();
    println!("Proof bundle: {}", output_dir.display());
    println!("Exit code:    {exit_code}");
}
