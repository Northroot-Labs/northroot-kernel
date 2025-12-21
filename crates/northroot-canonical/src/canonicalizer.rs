use canonical_json::to_string;
use serde_json::Value;

use crate::hygiene::{HygieneReport, HygieneStatus};
use crate::identifiers::ProfileId;
use std::collections::HashSet;

/// Error returned when canonicalization fails.
#[derive(thiserror::Error, Debug)]
pub enum CanonicalizationError {
    /// Provided JSON could not be canonicalized.
    #[error("invalid JSON structure: {0}")]
    InvalidStructure(String),
    /// A duplicate object member was detected.
    #[error("duplicate key detected at {0}")]
    DuplicateKey(String),
    /// Generic failure.
    #[error("other error: {0}")]
    Other(String),
}

/// Result of canonicalization.
pub struct CanonicalizationResult {
    /// Canonical UTF-8 bytes for the input value.
    pub bytes: Vec<u8>,
    /// Hygiene report describing strict-mode validation.
    pub report: HygieneReport,
}

/// Canonicalizer that emits deterministic bytes.
pub struct Canonicalizer {
    profile: ProfileId,
}

impl Canonicalizer {
    /// Creates a new canonicalizer for the provided profile.
    pub fn new(profile: ProfileId) -> Self {
        Self { profile }
    }

    /// Produces canonical bytes + hygiene report.
    pub fn canonicalize(
        &self,
        value: &Value,
    ) -> Result<CanonicalizationResult, CanonicalizationError> {
        self.assert_no_duplicates(value, "".to_string())?;
        let canonical =
            to_string(value).map_err(|err| CanonicalizationError::Other(err.to_string()))?;
        let bytes = canonical.into_bytes();
        let report = HygieneReport {
            status: HygieneStatus::Ok,
            warnings: vec![],
            metrics: Default::default(),
            profile_id: self.profile.clone(),
        };
        Ok(CanonicalizationResult { bytes, report })
    }

    fn assert_no_duplicates(
        &self,
        value: &Value,
        path: String,
    ) -> Result<(), CanonicalizationError> {
        if let Value::Object(map) = value {
            let mut seen = HashSet::new();
            for (key, child) in map {
                if !seen.insert(key) {
                    return Err(CanonicalizationError::DuplicateKey(format!(
                        "{}.{}",
                        path, key
                    )));
                }
                let child_path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", path, key)
                };
                self.assert_no_duplicates(child, child_path)?;
            }
        } else if let Value::Array(items) = value {
            for (idx, item) in items.iter().enumerate() {
                let item_path = format!("{}[{}]", path, idx);
                self.assert_no_duplicates(item, item_path)?;
            }
        }
        Ok(())
    }
}
