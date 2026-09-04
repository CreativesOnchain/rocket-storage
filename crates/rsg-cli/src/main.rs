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
//! rsg decode-key <KEY>                     # Decode bytes32 key into semantic path
//! ```
//!
//! Exit codes: 0 = PASS, 1 = FAIL, 2 = UNKNOWN, 3+ = tool error

mod args;
mod commands;

use anyhow::Result;
use clap::Parser;

use crate::args::{Cli, Commands};

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
        Commands::Capture { rpc_url, output } => {
            commands::capture::execute(&rpc_url, &output).await
        }
        Commands::Attest { fixture, rpc_url, manifest, review_record, output_dir } => {
            commands::attest::execute(
                fixture.as_deref(),
                rpc_url.as_deref(),
                &manifest,
                review_record.as_deref(),
                &output_dir,
            )
            .await
        }
        Commands::HashManifest { path } => commands::hash_manifest::execute(&path),
        Commands::ValidateFixture { path } => commands::validate_fixture::execute(&path),
        Commands::DecodeKey { key } => commands::decode_key::execute(&key),
    }
}
