//! Handler for `rsg hash-manifest`.

use anyhow::Result;
use std::path::Path;

/// Calculate and display SHA-256 digest of a manifest file.
pub fn execute(path: &Path) -> Result<i32> {
    let hash = rsg_attest::hash_file(path)?;
    println!("{hash}  {}", path.display());
    Ok(0)
}
