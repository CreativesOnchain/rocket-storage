//! Serialization and deserialization of frozen traces.

use std::path::Path;

use anyhow::{Context, Result};
use rsg_types::FrozenTrace;

/// Load a previously frozen trace from a JSON file (offline mode).
pub fn load_frozen_trace(path: &Path) -> Result<FrozenTrace> {
    let data = std::fs::read_to_string(path)
        .with_context(|| format!("cannot read fixture: {}", path.display()))?;
    serde_json::from_str(&data)
        .with_context(|| format!("cannot parse fixture: {}", path.display()))
}

/// Save a captured trace to a JSON file.
pub fn save_frozen_trace(trace: &FrozenTrace, path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(trace)?;
    std::fs::write(path, json)?;
    eprintln!("[rsg] Frozen trace saved to {}", path.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rsg_types::PinnedFixture;

    #[test]
    fn test_save_and_load_frozen_trace() {
        let path = std::env::temp_dir().join(format!("rsg_test_trace_{}.json", std::process::id()));

        let trace = FrozenTrace {
            pinned: PinnedFixture::default(),
            effects: vec![],
            external_calls: vec![],
        };

        save_frozen_trace(&trace, &path).expect("save should succeed");
        let loaded = load_frozen_trace(&path).expect("load should succeed");
        assert_eq!(trace, loaded);

        let _ = std::fs::remove_file(path);
    }
}
