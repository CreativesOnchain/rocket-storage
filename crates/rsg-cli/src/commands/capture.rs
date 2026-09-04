//! Handler for `rsg capture`.

use std::path::Path;

use anyhow::{Context, Result};
use rsg_capture::{capture_live, save_frozen_trace};

/// Execute trace capture from archive RPC node and persist frozen fixture.
pub async fn execute(rpc_url: &str, output: &Path) -> Result<i32> {
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
