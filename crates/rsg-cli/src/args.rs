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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_attest_with_fixture() {
        let cli =
            Cli::try_parse_from(["rsg", "attest", "--fixture", "fixtures/test.json"]).unwrap();

        match cli.command {
            Commands::Attest { fixture, rpc_url, manifest, review_record, output_dir } => {
                assert_eq!(fixture, Some(PathBuf::from("fixtures/test.json")));
                assert_eq!(rpc_url, None);
                assert_eq!(manifest, PathBuf::from("manifests/v1.4-mainnet/manifest.yaml"));
                assert_eq!(review_record, None);
                assert_eq!(output_dir, PathBuf::from("attestations/v1.4-mainnet"));
            }
            _ => panic!("expected Commands::Attest"),
        }
    }

    #[test]
    fn test_parse_attest_conflicts() {
        let err = Cli::try_parse_from([
            "rsg",
            "attest",
            "--fixture",
            "fixtures/test.json",
            "--rpc-url",
            "http://127.0.0.1:8545",
        ]);
        assert!(err.is_err());
    }

    #[test]
    fn test_parse_hash_manifest() {
        let cli = Cli::try_parse_from(["rsg", "hash-manifest", "manifest.yaml"]).unwrap();
        match cli.command {
            Commands::HashManifest { path } => {
                assert_eq!(path, PathBuf::from("manifest.yaml"));
            }
            _ => panic!("expected Commands::HashManifest"),
        }
    }

    #[test]
    fn test_parse_validate_fixture() {
        let cli = Cli::try_parse_from(["rsg", "validate-fixture", "trace.json"]).unwrap();
        match cli.command {
            Commands::ValidateFixture { path } => {
                assert_eq!(path, PathBuf::from("trace.json"));
            }
            _ => panic!("expected Commands::ValidateFixture"),
        }
    }

    #[test]
    fn test_parse_decode_key() {
        let cli = Cli::try_parse_from(["rsg", "decode-key", "0x123456"]).unwrap();
        match cli.command {
            Commands::DecodeKey { key } => {
                assert_eq!(key, "0x123456");
            }
            _ => panic!("expected Commands::DecodeKey"),
        }
    }

    #[test]
    fn test_parse_capture() {
        let cli =
            Cli::try_parse_from(["rsg", "capture", "--rpc-url", "http://127.0.0.1:8545"]).unwrap();
        match cli.command {
            Commands::Capture { rpc_url, output } => {
                assert_eq!(rpc_url, "http://127.0.0.1:8545");
                assert_eq!(output, PathBuf::from("fixtures/v1.4-mainnet/frozen-trace.json"));
            }
            _ => panic!("expected Commands::Capture"),
        }
    }
}
