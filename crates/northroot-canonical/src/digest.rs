use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::validation::ValidationError;

/// Supported digest algorithms for canonical identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DigestAlg {
    /// SHA-256 (the current Northroot default).
    #[serde(rename = "sha-256")]
    Sha256,
}

/// Algorithm + bytes digest, encoded as base64url without padding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Digest {
    /// Digest algorithm (currently always `sha-256`).
    pub alg: DigestAlg,
    /// Base64URL (no padding) digest bytes.
    #[serde(rename = "b64")]
    pub b64: String,
}

impl Digest {
    /// Constructs a validated digest.
    pub fn new(alg: DigestAlg, b64: impl Into<String>) -> Result<Self, ValidationError> {
        let b64 = b64.into();
        let re = Regex::new(r"^[A-Za-z0-9_-]{43,44}$").expect("invalid regex");
        if !re.is_match(&b64) {
            return Err(ValidationError::PatternMismatch {
                field: "digest",
                value: b64,
            });
        }
        Ok(Digest { alg, b64 })
    }
}
