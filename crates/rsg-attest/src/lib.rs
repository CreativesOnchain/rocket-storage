//! Proof bundle generator.
//!
//! Produces a deterministic JSON attestation and a human-readable Markdown
//! report, cryptographically binding the verdict to input hashes.

pub mod bundle;
pub mod canonical;
pub mod hash;
pub mod markdown;

pub use bundle::write_bundle;
pub use canonical::canonical_json;
pub use hash::{hash_file, sha256_hex};
pub use markdown::render_markdown;
