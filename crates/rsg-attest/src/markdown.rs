//! Markdown report renderer.

use rsg_types::{AttestationBundle, FrozenTrace, Manifest, Verdict};

/// Render the human-readable Markdown attestation report.
pub fn render_markdown(
    bundle: &AttestationBundle,
    trace: &FrozenTrace,
    _manifest: &Manifest,
) -> String {
    let mut md = String::new();

    // ── Header ────────────────────────────────────────────────────────────────
    md.push_str("# Rocket Pool RocketStorage Upgrade Attestation\n\n");
    md.push_str(&format!(
        "**Upgrade:** Rocket Pool v1.4 / Saturn 1  \n**Verdict:** `{}`\n\n",
        bundle.verdict.label()
    ));

    // ── Verdict box ───────────────────────────────────────────────────────────
    match &bundle.verdict {
        Verdict::Pass => {
            md.push_str(
                "> **✅ PASS** — All required RocketStorage effects and allowed external \
                 calls match the reviewed manifest exactly.\n\n",
            );
        }
        Verdict::Fail { reasons } => {
            md.push_str("> **❌ FAIL** — One or more required conditions are violated:\n\n");
            for r in reasons {
                md.push_str(&format!("> - `{}`\n", serde_json::to_string(r).unwrap_or_default()));
            }
            md.push('\n');
        }
        Verdict::Unknown { reasons } => {
            md.push_str(
                "> **⚠️ UNKNOWN** — The tool encountered a condition it cannot safely interpret:\n\n",
            );
            for r in reasons {
                md.push_str(&format!("> - `{}`\n", serde_json::to_string(r).unwrap_or_default()));
            }
            md.push('\n');
        }
    }

    // ── Pinned parameters ─────────────────────────────────────────────────────
    md.push_str("## Pinned Parameters\n\n");
    md.push_str("| Field | Value |\n|---|---|\n");
    md.push_str(&format!("| Chain ID | `{}` |\n", bundle.pinned.chain_id));
    md.push_str(&format!("| Pre-upgrade block | `{}` |\n", bundle.pinned.pre_block));
    md.push_str(&format!("| Pre-block hash | `{}` |\n", bundle.pinned.pre_block_hash));
    md.push_str(&format!("| Upgrade transaction | `{}` |\n", bundle.pinned.upgrade_tx));
    md.push_str(&format!("| Exec block | `{}` |\n", bundle.pinned.exec_block));
    md.push_str(&format!("| Upgrade contract | `{}` |\n", bundle.pinned.upgrade_contract));
    md.push_str(&format!("| RocketStorage | `{}` |\n", bundle.pinned.rocket_storage));
    md.push_str(&format!("| Source commit | `{}` |\n", bundle.pinned.source_commit));
    md.push('\n');

    // ── Input hashes ──────────────────────────────────────────────────────────
    md.push_str("## Input Hashes\n\n");
    md.push_str("| Artifact | SHA-256 |\n|---|---|\n");
    md.push_str(&format!("| Observed trace | `{}` |\n", bundle.hashes.observed_trace_sha256));
    md.push_str(&format!("| Manifest | `{}` |\n", bundle.hashes.manifest_sha256));
    md.push_str(&format!("| Review record | `{}` |\n", bundle.hashes.review_record_sha256));
    md.push_str(&format!("| Tool version | `{}` |\n", bundle.hashes.tool_version));
    md.push('\n');

    // ── Observed effects ──────────────────────────────────────────────────────
    md.push_str("## Observed RocketStorage Effects\n\n");
    if trace.effects.is_empty() {
        md.push_str("*No effects captured.*\n\n");
    } else {
        md.push_str("| # | Op | Key | Old Value | New Value |\n|---|---|---|---|---|\n");
        for eff in &trace.effects {
            let path = eff.semantic_path.as_deref().unwrap_or(&eff.raw_key);
            let old_trunc = truncate(&eff.old_value, 42);
            let new_trunc = truncate(&eff.new_value, 42);
            md.push_str(&format!(
                "| {} | `{:?}` | `{}` | `{}` | `{}` |\n",
                eff.call_index, eff.op, path, old_trunc, new_trunc
            ));
        }
        md.push('\n');

        // Effect count summary
        md.push_str("**Effect counts:**\n\n");
        for (op, count) in &bundle.effect_counts {
            md.push_str(&format!("- `{}`: {}\n", op, count));
        }
        md.push('\n');
    }

    // ── External calls ────────────────────────────────────────────────────────
    md.push_str("## External Calls\n\n");
    if trace.external_calls.is_empty() {
        md.push_str("*No external calls captured.*\n\n");
    } else {
        md.push_str("| # | From | To | Selector | ETH | Success |\n|---|---|---|---|---|---|\n");
        for call in &trace.external_calls {
            md.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | {} | {} |\n",
                call.call_index,
                truncate(&call.from, 12),
                truncate(&call.to, 12),
                call.selector,
                call.eth_value,
                if call.success { "✅" } else { "❌" }
            ));
        }
        md.push('\n');
    }

    // ── Trust model ───────────────────────────────────────────────────────────
    md.push_str("## Trust Model and Observation Boundary\n\n");
    md.push_str(&format!("{}\n\n", bundle.observation_boundary));
    md.push_str(&format!("> **Disclaimer:** {}\n\n", bundle.disclaimer));

    // ── Generation metadata ───────────────────────────────────────────────────
    if let Some(ts) = &bundle.generated_at {
        md.push_str(&format!("---\n*Generated at: {}*\n", ts));
    }

    md
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}
