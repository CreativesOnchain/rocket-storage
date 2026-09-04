//! Handler for `rsg hash-manifest`.

use std::path::Path;
use anyhow::Result;

/// Calculate and display SHA-256 digest of a manifest file.
pub fn execute(path: &Path) -> Result<i32> {
    let hash = rsg_attest::hash_file(path)?;
    println!("{hash}  {}", path.display());
    Ok(0)
}
