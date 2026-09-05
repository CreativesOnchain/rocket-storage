//! Markdown report renderer.

use indexmap::IndexMap;
use rsg_types::{
    AttestationBundle, AttestationHashes, FrozenTrace, Manifest, ObservedEffect,
    ObservedExternalCall, PinnedFixture, Verdict,
};

/// Render the human-readable Markdown attestation report.
pub fn render_markdown(
    bundle: &AttestationBundle,
    trace: &FrozenTrace,
    _manifest: &Manifest,
) -> String {
    let mut md = String::new();

    md.push_str(&render_header(bundle));
    md.push_str(&render_verdict_callout(&bundle.verdict));
    md.push_str(&render_pinned_parameters(&bundle.pinned));
    md.push_str(&render_input_hashes(&bundle.hashes));
    md.push_str(&render_observed_effects(
        &trace.effects,
        &bundle.effect_counts,
    ));
    md.push_str(&render_external_calls(&trace.external_calls));
    md.push_str(&render_trust_model(
        &bundle.observation_boundary,
        &bundle.disclaimer,
    ));
    md.push_str(&render_footer(bundle.generated_at.as_deref()));

    md
}

fn render_header(bundle: &AttestationBundle) -> String {
    format!(
        "# Rocket Pool RocketStorage Upgrade Attestation\n\n\
         **Upgrade:** Rocket Pool v1.4 / Saturn 1  \n\
         **Verdict:** `{}`\n\n",
        bundle.verdict.label()
    )
}

fn render_verdict_callout(verdict: &Verdict) -> String {
    let mut out = String::new();
    match verdict {
        Verdict::Pass => {
            out.push_str(
                "> **✅ PASS** — All required RocketStorage effects and allowed external \
                 calls match the reviewed manifest exactly.\n\n",
            );
        }
        Verdict::Fail { reasons } => {
            out.push_str("> **❌ FAIL** — One or more required conditions are violated:\n\n");
            for r in reasons {
                out.push_str(&format!(
                    "> - `{}`\n",
                    serde_json::to_string(r).unwrap_or_default()
                ));
            }
            out.push('\n');
        }
        Verdict::Unknown { reasons } => {
            out.push_str(
                "> **⚠️ UNKNOWN** — The tool encountered a condition it cannot safely interpret:\n\n",
            );
            for r in reasons {
                out.push_str(&format!(
                    "> - `{}`\n",
                    serde_json::to_string(r).unwrap_or_default()
                ));
            }
            out.push('\n');
        }
    }
    out
}

fn render_pinned_parameters(pinned: &PinnedFixture) -> String {
    let mut out = String::new();
    out.push_str("## Pinned Parameters\n\n");
    out.push_str("| Field | Value |\n|---|---|\n");
    out.push_str(&format!("| Chain ID | `{}` |\n", pinned.chain_id));
    out.push_str(&format!("| Pre-upgrade block | `{}` |\n", pinned.pre_block));
    out.push_str(&format!(
        "| Pre-block hash | `{}` |\n",
        pinned.pre_block_hash
    ));
    out.push_str(&format!(
        "| Upgrade transaction | `{}` |\n",
        pinned.upgrade_tx
    ));
    out.push_str(&format!("| Exec block | `{}` |\n", pinned.exec_block));
    out.push_str(&format!(
        "| Upgrade contract | `{}` |\n",
        pinned.upgrade_contract
    ));
    out.push_str(&format!(
        "| RocketStorage | `{}` |\n",
        pinned.rocket_storage
    ));
    out.push_str(&format!("| Source commit | `{}` |\n", pinned.source_commit));
    out.push('\n');
    out
}

fn render_input_hashes(hashes: &AttestationHashes) -> String {
    let mut out = String::new();
    out.push_str("## Input Hashes\n\n");
    out.push_str("| Artifact | SHA-256 |\n|---|---|\n");
    out.push_str(&format!(
        "| Observed trace | `{}` |\n",
        hashes.observed_trace_sha256
    ));
    out.push_str(&format!("| Manifest | `{}` |\n", hashes.manifest_sha256));
    out.push_str(&format!(
        "| Review record | `{}` |\n",
        hashes.review_record_sha256
    ));
    out.push_str(&format!("| Tool version | `{}` |\n", hashes.tool_version));
    out.push('\n');
    out
}

fn render_observed_effects(
    effects: &[ObservedEffect],
    effect_counts: &IndexMap<String, usize>,
) -> String {
    let mut out = String::new();
    out.push_str("## Observed RocketStorage Effects\n\n");

    if effects.is_empty() {
        out.push_str("*No effects captured.*\n\n");
        return out;
    }

    out.push_str("| # | Op | Key | Old Value | New Value |\n|---|---|---|---|---|\n");
    for eff in effects {
        let path = eff.semantic_path.as_deref().unwrap_or(&eff.raw_key);
        let old_trunc = truncate(&eff.old_value, 42);
        let new_trunc = truncate(&eff.new_value, 42);
        out.push_str(&format!(
            "| {} | `{:?}` | `{}` | `{}` | `{}` |\n",
            eff.call_index, eff.op, path, old_trunc, new_trunc
        ));
    }
    out.push('\n');

    out.push_str("**Effect counts:**\n\n");
    for (op, count) in effect_counts {
        out.push_str(&format!("- `{}`: {}\n", op, count));
    }
    out.push('\n');

    out
}

fn render_external_calls(external_calls: &[ObservedExternalCall]) -> String {
    let mut out = String::new();
    out.push_str("## External Calls\n\n");

    if external_calls.is_empty() {
        out.push_str("*No external calls captured.*\n\n");
        return out;
    }

    out.push_str("| # | From | To | Selector | ETH | Success |\n|---|---|---|---|---|---|\n");
    for call in external_calls {
        out.push_str(&format!(
            "| {} | `{}` | `{}` | `{}` | {} | {} |\n",
            call.call_index,
            truncate(&call.from, 12),
            truncate(&call.to, 12),
            call.selector,
            call.eth_value,
            if call.success { "✅" } else { "❌" }
        ));
    }
    out.push('\n');

    out
}

fn render_trust_model(boundary: &str, disclaimer: &str) -> String {
    format!(
        "## Trust Model and Observation Boundary\n\n\
         {}\n\n\
         > **Disclaimer:** {}\n\n",
        boundary, disclaimer
    )
}

fn render_footer(generated_at: Option<&str>) -> String {
    match generated_at {
        Some(ts) => format!("---\n*Generated at: {}*\n", ts),
        None => String::new(),
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello", 10), "hello");
        assert_eq!(truncate("hello world", 5), "hello…");
    }

    #[test]
    fn test_render_verdict_callout() {
        let pass = render_verdict_callout(&Verdict::Pass);
        assert!(pass.contains("✅ PASS"));

        let fail = render_verdict_callout(&Verdict::Fail {
            reasons: vec![rsg_types::FailReason::WrongValue {
                semantic_path: "test".into(),
                field: "new_value".into(),
                expected: "1".into(),
                observed: "2".into(),
            }],
        });
        assert!(fail.contains("❌ FAIL"));

        let unknown = render_verdict_callout(&Verdict::Unknown {
            reasons: vec![rsg_types::UnknownReason::UndecodeableKey {
                raw_key: "0x12".into(),
                op: "SetUint".into(),
            }],
        });
        assert!(unknown.contains("⚠️ UNKNOWN"));
    }

    #[test]
    fn test_render_empty_sections() {
        let effects_md = render_observed_effects(&[], &IndexMap::new());
        assert!(effects_md.contains("*No effects captured.*"));

        let calls_md = render_external_calls(&[]);
        assert!(calls_md.contains("*No external calls captured.*"));
    }
}
