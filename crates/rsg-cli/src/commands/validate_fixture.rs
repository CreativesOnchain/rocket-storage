//! Handler for `rsg validate-fixture`.

use std::path::Path;

use anyhow::Result;
use rsg_capture::load_frozen_trace;

const EXPECTED_TX: &str =
    "0x2fc10aad3c1b00bdfa9b6fddab79e0f2688609848f8f7a1a6449ab42da38530c";
const EXPECTED_BLOCK: u64 = 24_479_994;
const EXPECTED_CHAIN: u64 = 1;

/// Validate pinned fixture parameters against protocol deployment invariants.
pub fn execute(path: &Path) -> Result<i32> {
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

    let mut ok = true;
    if p.chain_id != EXPECTED_CHAIN {
        eprintln!("  ❌ chain_id mismatch (expected {EXPECTED_CHAIN})");
        ok = false;
    }
    if p.upgrade_tx.to_lowercase() != EXPECTED_TX {
        eprintln!("  ❌ upgrade_tx mismatch");
        ok = false;
    }
    if p.exec_block != EXPECTED_BLOCK {
        eprintln!("  ❌ exec_block mismatch (expected {EXPECTED_BLOCK})");
        ok = false;
    }

    if ok {
        println!("  ✅ All pinned parameters match.");
        Ok(0)
    } else {
        Ok(1)
    }
}
