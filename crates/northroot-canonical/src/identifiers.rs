use crate::digest::Digest;
use crate::validation::ValidationError;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// Opaque reference to content-addressed bytes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentRef {
    /// Digest that identifies the referenced bytes.
    pub digest: Digest,
    /// Optional size hint; does not affect hashing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
    /// Optional media type hint (e.g., `application/json`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

macro_rules! newtype {
    ($name:ident, $doc:expr, $pattern:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            /// Creates a new instance without validation; callers are responsible for conformity.
            pub fn new(value: String) -> Self {
                Self(value)
            }

            /// Parses a validated identifier from a string.
            pub fn parse(value: impl Into<String>) -> Result<Self, ValidationError> {
                let s = value.into();
                if !Regex::new($pattern).expect("invalid regex").is_match(&s) {
                    return Err(ValidationError::PatternMismatch {
                        field: stringify!($name),
                        value: s,
                    });
                }
                Ok(Self(s))
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                Self(value)
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }
    };
}

newtype!(
    ProfileId,
    "Identifier for canonicalization profiles (pattern: `[A-Za-z0-9_-]{16,128}`)",
    r"^[A-Za-z0-9_-]{16,128}$"
);
newtype!(
    PrincipalId,
    "Stable identifier for principals (`kind:name`, lowercase, URL-safe).",
    r"^(human|service|agent|org):[a-z][a-z0-9_-]{0,62}$"
);
newtype!(
    ToolName,
    "Canonical tool identifier like `canon.hash` or `llm.generate`.",
    r"^[a-z][a-z0-9_]*([.][a-z][a-z0-9_]*){0,7}$"
);
newtype!(
    Timestamp,
    "UTC RFC3339 timestamp with `Z` suffix.",
    r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(\.\d{1,9})?Z$"
);
