//! Verdicts and failure reasons for upgrade verification.

use serde::{Deserialize, Serialize};

/// A specific reason for a FAIL verdict.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum FailReason {
    /// A required manifest entry was never observed.
    MissingRequiredEffect { semantic_path: String },
    /// An observed mutation is not in the manifest.
    UndeclaredWrite { raw_key: String, op: String, new_value: String },
    /// Observed old or new value does not match manifest expectation.
    WrongValue { semantic_path: String, field: String, expected: String, observed: String },
    /// The op (storage type) does not match.
    TypeDrift { raw_key: String, expected_op: String, observed_op: String },
    /// A key appeared more times than declared.
    DuplicateMutation { semantic_path: String, expected: usize, observed: usize },
    /// An expected deletion was not observed.
    OmittedDeletion { semantic_path: String },
    /// Two contract addresses appear to be swapped.
    SwappedAddress { path_a: String, path_b: String },
    /// An external call target, selector, value, or multiplicity is wrong.
    UnexpectedExternalCall { to: String, selector: String, reason: String },
}

/// A specific reason for an UNKNOWN verdict.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UnknownReason {
    /// A raw key could not be decoded to a semantic path.
    UndecodeableKey { raw_key: String, op: String },
    /// The call shape is not a supported mutator form.
    UnsupportedCallShape { call_index: usize, selector: String },
    /// The trace is incomplete or sourced from an unexpected address.
    TraceIncomplete { detail: String },
}

/// The overall verification verdict.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "verdict", rename_all = "UPPERCASE")]
pub enum Verdict {
    Pass,
    Fail { reasons: Vec<FailReason> },
    Unknown { reasons: Vec<UnknownReason> },
}

impl Verdict {
    /// Process exit code: 0 on PASS, 1 on FAIL, 2 on UNKNOWN.
    pub fn exit_code(&self) -> i32 {
        match self {
            Verdict::Pass => 0,
            Verdict::Fail { .. } => 1,
            Verdict::Unknown { .. } => 2,
        }
    }

    /// String label for logging and reports.
    pub fn label(&self) -> &'static str {
        match self {
            Verdict::Pass => "PASS",
            Verdict::Fail { .. } => "FAIL",
            Verdict::Unknown { .. } => "UNKNOWN",
        }
    }
}
