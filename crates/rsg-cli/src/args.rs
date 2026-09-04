//! Command-line argument parser definitions using clap.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "rsg",
    about = "Rocket Pool RocketStorage Upgrade Effects Gate",
    long_about = "Deterministically replays a Rocket Pool upgrade transaction and \
                  verifies that the RocketStorage mutations match a reviewed manifest.\n\n\
                  Exit codes: 0=PASS  1=FAIL  2=UNKNOWN  3=tool-error",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Capture the live upgrade trace from an archive RPC and save a frozen fixture.
    Capture {
        /// Archive RPC URL (must support debug_traceTransaction).
        #[arg(long, env = "ETH_RPC_URL")]
        rpc_url: String,

        /// Path to write the frozen trace JSON.
        #[arg(long, default_value = "fixtures/v1.4-mainnet/frozen-trace.json")]
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
