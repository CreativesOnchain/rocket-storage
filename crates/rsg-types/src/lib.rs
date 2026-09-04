//! Shared types for rocket-storage-gate (`rsg`).
//!
//! All domain models are organized into focused modules and re-exported
//! at the crate root for convenience:
//! - [`fixture`]: Pinned environment parameters.
//! - [`storage_op`]: RocketStorage function selectors and operations.
//! - [`trace`]: Observed effects and frozen trace definitions.
//! - [`manifest`]: Expected effects and upgrade specification manifest.
//! - [`verdict`]: Verification verdicts and failure/unknown reasons.
//! - [`attestation`]: Machine-readable proof bundle and input hashes.

pub mod attestation;
pub mod fixture;
pub mod manifest;
pub mod storage_op;
pub mod trace;
pub mod verdict;

// Re-export all primary types at the crate root
pub use attestation::{AttestationBundle, AttestationHashes};
pub use fixture::PinnedFixture;
pub use manifest::{Manifest, ManifestEffect, ManifestExternalCall, RequirementLevel};
pub use storage_op::StorageOp;
pub use trace::{FrozenTrace, ObservedEffect, ObservedExternalCall};
pub use verdict::{FailReason, UnknownReason, Verdict};
