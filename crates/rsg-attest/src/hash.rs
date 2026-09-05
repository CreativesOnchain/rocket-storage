//! Cryptographic hashing utilities.

use anyhow::Result;
use hex::encode;
use sha2::{Digest, Sha256};
use std::fs::read;
use std::path::Path;

/// Compute the lowercase hexadecimal SHA-256 digest of byte data.
pub fn sha256_hex(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    encode(hasher.finalize())
}

/// Compute the lowercase hexadecimal SHA-256 digest of a file on disk.
pub fn hash_file(path: &Path) -> Result<String> {
    let data = read(path)?;
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
