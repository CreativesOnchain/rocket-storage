//! Cryptographic hashing utilities.

use std::path::Path;
use anyhow::Result;
use sha2::{Digest, Sha256};

/// Compute the lowercase hexadecimal SHA-256 digest of byte data.
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}

/// Compute the lowercase hexadecimal SHA-256 digest of a file on disk.
pub fn hash_file(path: &Path) -> Result<String> {
    let data = std::fs::read(path)?;
    Ok(sha256_hex(&data))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_is_stable() {
        let data = b"test input";
        let h1 = sha256_hex(data);
        let h2 = sha256_hex(data);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }
}
