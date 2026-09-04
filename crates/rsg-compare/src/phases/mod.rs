//! Evaluation phases of the comparison engine.

pub mod calls;
pub mod effects;
pub mod pinned;
pub mod swapped;

pub use calls::check_external_calls;
pub use effects::{
    check_effect_against_entry, check_manifest_effects, check_undeclared_writes,
    scan_observed_effects,
};
pub use pinned::validate_pinned;
pub use swapped::detect_swapped_addresses;
