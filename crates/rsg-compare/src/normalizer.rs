//! Value normalization and deletion checks for storage comparison.

/// Normalise a value string for comparison: lowercase, trim whitespace,
/// and canonicalize the zero address representation.
pub fn normalise_value(v: &str) -> String {
    let v = v.trim().to_lowercase();
    if v == "0x0000000000000000000000000000000000000000" {
        return "0x0".to_string();
    }
    v
}

/// Check if a new value qualifies as a valid zero/empty value for a delete operation.
pub fn is_valid_deletion_target(value: &str) -> bool {
    let trimmed = value.trim();
    matches!(
        trimmed,
        "0" | "false"
            | ""
            | "\"\""
            | "0x"
            | "0x0000000000000000000000000000000000000000"
            | "0x0000000000000000000000000000000000000000000000000000000000000000"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalise_value() {
        assert_eq!(normalise_value("0xABCD"), "0xabcd");
        assert_eq!(normalise_value("0x0000000000000000000000000000000000000000"), "0x0");
        assert_eq!(normalise_value(" 42 "), "42");
    }

    #[test]
    fn test_is_valid_deletion_target() {
        assert!(is_valid_deletion_target("0"));
        assert!(is_valid_deletion_target("false"));
        assert!(is_valid_deletion_target(""));
        assert!(is_valid_deletion_target("\"\""));
        assert!(is_valid_deletion_target("0x"));
        assert!(is_valid_deletion_target("0x0000000000000000000000000000000000000000"));
        assert!(is_valid_deletion_target(
            "0x0000000000000000000000000000000000000000000000000000000000000000"
        ));
        assert!(!is_valid_deletion_target("1"));
        assert!(!is_valid_deletion_target("true"));
        assert!(!is_valid_deletion_target("0x1234"));
    }
}
