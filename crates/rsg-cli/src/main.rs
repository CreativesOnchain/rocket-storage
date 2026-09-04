//! `rsg` — Rocket Pool RocketStorage Upgrade Effects Gate
//!
//! A standalone CLI that deterministically replays a Rocket Pool protocol
//! upgrade and verifies whether the transaction produced exactly the declared
//! RocketStorage changes.
//!
//! ## Commands
//!
//! ```text
//! rsg capture --rpc-url <URL>              # Capture live trace, save frozen fixture
//! rsg attest  --fixture <PATH>             # Offline: load fixture, attest
//! rsg attest  --rpc-url <URL>              # Live: capture + attest in one step
//! rsg hash-manifest <PATH>                 # Print manifest SHA-256
//! rsg validate-fixture <PATH>              # Validate pinned params
//! ```
//!
//! Exit codes: 0 = PASS, 1 = FAIL, 2 = UNKNOWN, 3+ = tool error

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use rsg_attest::write_bundle;
use rsg_capture::{capture_live, load_frozen_trace, save_frozen_trace};
use rsg_compare::compare;
use rsg_types::{FrozenTrace, Manifest, Verdict};
use std::path::{Path, PathBuf};

// ─── CLI definition ───────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "rsg",
    about = "Rocket Pool RocketStorage Upgrade Effects Gate",
    long_about = "Deterministically replays a Rocket Pool upgrade transaction and \
                  verifies that the RocketStorage mutations match a reviewed manifest.\n\n\
                  Exit codes: 0=PASS  1=FAIL  2=UNKNOWN  3=tool-error",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Capture the live upgrade trace from an archive RPC and save a frozen fixture.
    Capture {
        /// Archive RPC URL (must support debug_traceTransaction).
        #[arg(long, env = "ETH_RPC_URL")]
        rpc_url: String,

        /// Path to write the frozen trace JSON.
        #[arg(
            long,
            default_value = "fixtures/v1.4-mainnet/frozen-trace.json"
        )]
        output: PathBuf,
    },

    /// Attest: compare a frozen trace against the manifest and produce a proof bundle.
    Attest {
        /// Path to frozen trace JSON (offline mode).
        #[arg(long, conflicts_with = "rpc_url")]
        fixture: Option<PathBuf>,

        /// Archive RPC URL (live mode — capture + attest in one step).
        #[arg(long, env = "ETH_RPC_URL", conflicts_with = "fixture")]
        rpc_url: Option<String>,

        /// Path to manifest YAML.
        #[arg(long, default_value = "manifests/v1.4-mainnet/manifest.yaml")]
        manifest: PathBuf,

        /// Path to review-record JSON (optional).
        #[arg(long)]
        review_record: Option<PathBuf>,

        /// Directory to write the proof bundle.
        #[arg(long, default_value = "attestations/v1.4-mainnet")]
        output_dir: PathBuf,
    },

    /// Print the SHA-256 hash of a manifest file (for review-record authoring).
    HashManifest {
        /// Path to the manifest YAML or JSON.
        path: PathBuf,
    },

    /// Validate pinned fixture parameters (chain ID, block hash, tx hash).
    ValidateFixture {
        /// Path to frozen trace JSON.
        path: PathBuf,
    },

    /// Decode a raw bytes32 RocketStorage key into its semantic path.
    DecodeKey {
        /// Raw key (hex string "0x..." or 64 hex characters).
        key: String,
    },
}

// ─── Main ─────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(code) => std::process::exit(code),
        Err(e) => {
            eprintln!("[rsg] ERROR: {e:#}");
            std::process::exit(3);
        }
    }
}

async fn run(cli: Cli) -> Result<i32> {
    match cli.command {
        Commands::Capture { rpc_url, output } => cmd_capture(&rpc_url, &output).await,
        Commands::Attest {
            fixture,
            rpc_url,
            manifest,
            review_record,
            output_dir,
        } => {
            cmd_attest(fixture.as_deref(), rpc_url.as_deref(), &manifest, review_record.as_deref(), &output_dir).await
        }
        Commands::HashManifest { path } => cmd_hash_manifest(&path),
        Commands::ValidateFixture { path } => cmd_validate_fixture(&path),
        Commands::DecodeKey { key } => {
            let cat = rsg_decode::KeyCatalogue::build();
            match cat.lookup_hex(&key) {
                Some(path) => {
                    println!("Key:           {key}");
                    println!("Semantic path: {path}");
                    Ok(0)
                }
                None => {
                    eprintln!("Key: {key}");
                    eprintln!("Status: UNKNOWN (not in catalogue)");
                    Ok(2)
                }
            }
        }
    }
}

// ─── Commands ─────────────────────────────────────────────────────────────────

async fn cmd_capture(rpc_url: &str, output: &Path) -> Result<i32> {
    eprintln!("[rsg] Capturing Saturn 1 upgrade trace…");
    let trace = capture_live(rpc_url)
        .await
        .context("capture failed")?;
    save_frozen_trace(&trace, output)?;
    println!("Frozen trace saved to {}", output.display());
    println!("Effects captured: {}", trace.effects.len());
    println!("External calls:  {}", trace.external_calls.len());
    Ok(0)
}

async fn cmd_attest(
    fixture: Option<&Path>,
    rpc_url: Option<&str>,
    manifest_path: &Path,
    review_record: Option<&Path>,
    output_dir: &Path,
) -> Result<i32> {
    // ── Load trace ────────────────────────────────────────────────────────────
    let trace: FrozenTrace = match (fixture, rpc_url) {
        (Some(path), _) => {
            eprintln!("[rsg] Loading frozen trace from {}…", path.display());
            load_frozen_trace(path)?
        }
        (_, Some(url)) => {
            eprintln!("[rsg] Capturing live trace…");
            capture_live(url).await?
        }
        _ => anyhow::bail!("must specify either --fixture or --rpc-url"),
    };

    // ── Load manifest ─────────────────────────────────────────────────────────
    eprintln!("[rsg] Loading manifest from {}…", manifest_path.display());
    let manifest_raw = std::fs::read_to_string(manifest_path)
        .with_context(|| format!("cannot read manifest: {}", manifest_path.display()))?;
    let manifest: Manifest = serde_yaml::from_str(&manifest_raw)
        .context("manifest parse error")?;

    // ── Compare ───────────────────────────────────────────────────────────────
    eprintln!("[rsg] Running comparator…");
    let verdict = compare(&trace, &manifest);

    // ── Write proof bundle ────────────────────────────────────────────────────
    write_bundle(
        &trace,
        &manifest,
        &manifest_raw,
        review_record,
        &verdict,
        output_dir,
    )?;

    // ── Print result ──────────────────────────────────────────────────────────
    let code = verdict.exit_code();

    eprintln!();
    match &verdict {
        Verdict::Pass => {
            println!("✅  PASS");
            println!("    All {} effects match the manifest.", trace.effects.len());
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
    println!("Exit code:    {code}");

    Ok(code)
}

fn cmd_hash_manifest(path: &Path) -> Result<i32> {
    let hash = rsg_attest::hash_file(path)?;
    println!("{hash}  {}", path.display());
    Ok(0)
}

fn cmd_validate_fixture(path: &Path) -> Result<i32> {
    let trace = load_frozen_trace(path)?;
    let p = &trace.pinned;

    println!("Fixture validation:");
    println!("  chain_id:         {}", p.chain_id);
    println!("  pre_block:        {}", p.pre_block);
    println!("  pre_block_hash:   {}", p.pre_block_hash);
    println!("  upgrade_tx:       {}", p.upgrade_tx);
    println!("  exec_block:       {}", p.exec_block);
    println!("  upgrade_contract: {}", p.upgrade_contract);
    println!("  rocket_storage:   {}", p.rocket_storage);
    println!("  source_commit:    {}", p.source_commit);

    let expected_tx =
        "0x2fc10aad3c1b00bdfa9b6fddab79e0f2688609848f8f7a1a6449ab42da38530c";
    let expected_block = 24_479_994u64;
    let expected_chain = 1u64;

    let mut ok = true;
    if p.chain_id != expected_chain {
        eprintln!("  ❌ chain_id mismatch (expected {expected_chain})");
        ok = false;
    }
    if p.upgrade_tx.to_lowercase() != expected_tx {
        eprintln!("  ❌ upgrade_tx mismatch");
        ok = false;
    }
    if p.exec_block != expected_block {
        eprintln!("  ❌ exec_block mismatch (expected {expected_block})");
        ok = false;
    }

    if ok {
        println!("  ✅ All pinned parameters match.");
        Ok(0)
    } else {
        Ok(1)
    }
}
