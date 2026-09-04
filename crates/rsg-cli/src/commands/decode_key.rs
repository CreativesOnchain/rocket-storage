//! Handler for `rsg decode-key`.

use anyhow::Result;
use rsg_decode::KeyCatalogue;

/// Decode a raw bytes32 storage key into its human-readable semantic path.
pub fn execute(key: &str) -> Result<i32> {
    let cat = KeyCatalogue::build();
    match cat.lookup_hex(key) {
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
