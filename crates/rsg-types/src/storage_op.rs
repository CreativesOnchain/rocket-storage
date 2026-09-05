//! Storage operations and function selectors for RocketStorage.

use serde::{Deserialize, Serialize};

/// Which RocketStorage typed setter/deleter was called.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum StorageOp {
    #[serde(alias = "setAddress")]
    SetAddress,
    #[serde(alias = "setBool")]
    SetBool,
    #[serde(alias = "setBytes")]
    SetBytes,
    #[serde(alias = "setBytes32")]
    SetBytes32,
    #[serde(alias = "setInt")]
    SetInt,
    #[serde(alias = "setString")]
    SetString,
    #[serde(alias = "setUint")]
    SetUint,
    #[serde(alias = "deleteAddress")]
    DeleteAddress,
    #[serde(alias = "deleteBool")]
    DeleteBool,
    #[serde(alias = "deleteBytes")]
    DeleteBytes,
    #[serde(alias = "deleteBytes32")]
    DeleteBytes32,
    #[serde(alias = "deleteInt")]
    DeleteInt,
    #[serde(alias = "deleteString")]
    DeleteString,
    #[serde(alias = "deleteUint")]
    DeleteUint,
    #[serde(alias = "addUint")]
    AddUint,
    #[serde(alias = "subUint")]
    SubUint,
}

impl StorageOp {
    /// 4-byte selector (big-endian hex without 0x prefix).
    pub fn selector_hex(&self) -> &'static str {
        match self {
            StorageOp::SetAddress => "ca446dd9",
            StorageOp::SetBool => "abfdcced",
            StorageOp::SetBytes => "2e28d084",
            StorageOp::SetBytes32 => "4e91db08",
            StorageOp::SetInt => "3e49bed0",
            StorageOp::SetString => "6e899550",
            StorageOp::SetUint => "e2a4853a",
            StorageOp::DeleteAddress => "0e14a376",
            StorageOp::DeleteBool => "2c62ff2d",
            StorageOp::DeleteBytes => "616b59f6",
            StorageOp::DeleteBytes32 => "0b9adc57",
            StorageOp::DeleteInt => "8c160095",
            StorageOp::DeleteString => "f6bb3cc4",
            StorageOp::DeleteUint => "e2b202bf",
            StorageOp::AddUint => "adb353dc",
            StorageOp::SubUint => "ebb9d8c9",
        }
    }

    /// Return whether this op deletes a value (new_value will be zero/empty).
    pub fn is_delete(&self) -> bool {
        matches!(
            self,
            StorageOp::DeleteAddress
                | StorageOp::DeleteBool
                | StorageOp::DeleteBytes
                | StorageOp::DeleteBytes32
                | StorageOp::DeleteInt
                | StorageOp::DeleteString
                | StorageOp::DeleteUint
        )
    }

    /// Try to parse from the 4-byte hex selector string (without 0x).
    pub fn from_selector(sel: &str) -> Option<Self> {
        let sel = sel.trim_start_matches("0x").to_lowercase();
        match sel.as_str() {
            "ca446dd9" => Some(StorageOp::SetAddress),
            "abfdcced" => Some(StorageOp::SetBool),
            "2e28d084" => Some(StorageOp::SetBytes),
            "4e91db08" => Some(StorageOp::SetBytes32),
            "3e49bed0" => Some(StorageOp::SetInt),
            "6e899550" => Some(StorageOp::SetString),
            "e2a4853a" => Some(StorageOp::SetUint),
            "0e14a376" => Some(StorageOp::DeleteAddress),
            "2c62ff2d" => Some(StorageOp::DeleteBool),
            "616b59f6" => Some(StorageOp::DeleteBytes),
            "0b9adc57" => Some(StorageOp::DeleteBytes32),
            "8c160095" => Some(StorageOp::DeleteInt),
            "f6bb3cc4" => Some(StorageOp::DeleteString),
            "e2b202bf" => Some(StorageOp::DeleteUint),
            "adb353dc" => Some(StorageOp::AddUint),
            "ebb9d8c9" => Some(StorageOp::SubUint),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL_OPS: &[StorageOp] = &[
        StorageOp::SetAddress,
        StorageOp::SetBool,
        StorageOp::SetBytes,
        StorageOp::SetBytes32,
        StorageOp::SetInt,
        StorageOp::SetString,
        StorageOp::SetUint,
        StorageOp::DeleteAddress,
        StorageOp::DeleteBool,
        StorageOp::DeleteBytes,
        StorageOp::DeleteBytes32,
        StorageOp::DeleteInt,
        StorageOp::DeleteString,
        StorageOp::DeleteUint,
        StorageOp::AddUint,
        StorageOp::SubUint,
    ];

    #[test]
    fn test_selector_roundtrip_all_variants() {
        for op in ALL_OPS {
            let hex = op.selector_hex();
            assert_eq!(
                StorageOp::from_selector(hex),
                Some(*op),
                "failed roundtrip for {op:?} with selector {hex}"
            );

            // Test with 0x prefix
            let prefixed = format!("0x{hex}");
            assert_eq!(
                StorageOp::from_selector(&prefixed),
                Some(*op),
                "failed prefixed roundtrip for {op:?}"
            );

            // Test case insensitivity
            let uppercase = hex.to_uppercase();
            assert_eq!(
                StorageOp::from_selector(&uppercase),
                Some(*op),
                "failed uppercase roundtrip for {op:?}"
            );
        }
    }

    #[test]
    fn test_is_delete_classification() {
        for op in ALL_OPS {
            let name = format!("{op:?}");
            let should_delete = name.starts_with("Delete");
            assert_eq!(op.is_delete(), should_delete, "is_delete mismatch for {op:?}");
        }
    }

    #[test]
    fn test_unknown_selector_returns_none() {
        assert_eq!(StorageOp::from_selector("ffffffff"), None);
        assert_eq!(StorageOp::from_selector("0x00000000"), None);
        assert_eq!(StorageOp::from_selector(""), None);
    }

    #[test]
    fn test_serde_serialization_and_alias() {
        let op = StorageOp::SetAddress;
        let json = serde_json::to_string(&op).unwrap();
        assert_eq!(json, "\"SetAddress\"");

        // Test alias deserialization (camelCase)
        let deserialized: StorageOp = serde_json::from_str("\"setAddress\"").unwrap();
        assert_eq!(deserialized, StorageOp::SetAddress);
    }
}
