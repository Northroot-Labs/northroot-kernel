use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::validation::ValidationError;

const DECIMAL_SCALE_MAX: u32 = 18;

/// Neutral numeric quantities for canonical events.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "t")]
pub enum Quantity {
    /// Fixed-point decimal (`Dec`).
    #[serde(rename = "dec")]
    Dec {
        /// Signed base-10 mantissa (minimal form; no leading zeros; `"-0"` forbidden).
        m: String,
        /// Non-negative scale (default max 18, configurable per schema).
        s: u32,
    },
    /// Integer quantity (`Int`).
    #[serde(rename = "int")]
    Int {
        /// Signed integer string (no leading zeros except `"0"`).
        v: String,
    },
    /// Rational quantity (`Rat`).
    #[serde(rename = "rat")]
    Rat {
        /// Signed numerator (reduced, no `"-0"`).
        n: String,
        /// Positive denominator (`> 0`).
        d: String,
    },
    /// Explicit IEEE-754 binary float (`F64`), lossy opt-in only.
    #[serde(rename = "f64")]
    F64 {
        /// Hex-encoded IEEE-754 bit pattern (lowercase).
        bits: String,
    },
}

impl Quantity {
    /// Constructs a validated decimal quantity.
    pub fn dec(mantissa: impl Into<String>, scale: u32) -> Result<Self, ValidationError> {
        let mantissa = mantissa.into();
        if scale > DECIMAL_SCALE_MAX {
            return Err(ValidationError::OutOfBounds {
                field: "scale",
                value: scale.to_string(),
            });
        }
        if !is_valid_integer(&mantissa) {
            return Err(ValidationError::PatternMismatch {
                field: "mantissa",
                value: mantissa,
            });
        }
        Ok(Quantity::Dec {
            m: mantissa,
            s: scale,
        })
    }

    /// Constructs a validated integer quantity.
    pub fn int(value: impl Into<String>) -> Result<Self, ValidationError> {
        let value = value.into();
        if !is_valid_integer(&value) {
            return Err(ValidationError::PatternMismatch {
                field: "int",
                value,
            });
        }
        Ok(Quantity::Int { v: value })
    }

    /// Constructs a rational quantity.
    pub fn rat(
        numerator: impl Into<String>,
        denominator: impl Into<String>,
    ) -> Result<Self, ValidationError> {
        let numerator = numerator.into();
        let denominator = denominator.into();
        if !is_valid_integer(&numerator) {
            return Err(ValidationError::PatternMismatch {
                field: "rat_numerator",
                value: numerator,
            });
        }
        if !is_valid_positive_integer(&denominator) {
            return Err(ValidationError::PatternMismatch {
                field: "rat_denominator",
                value: denominator,
            });
        }
        Ok(Quantity::Rat {
            n: numerator,
            d: denominator,
        })
    }

    /// Constructs a validated IEEE-754 encoding.
    pub fn f64(bits: impl Into<String>) -> Result<Self, ValidationError> {
        let bits = bits.into();
        let re = Regex::new(r"^[0-9a-f]{16}$").expect("invalid regex");
        if !re.is_match(&bits) {
            return Err(ValidationError::PatternMismatch {
                field: "f64",
                value: bits,
            });
        }
        Ok(Quantity::F64 { bits })
    }
}

fn is_valid_integer(value: &str) -> bool {
    if value == "0" {
        return true;
    }
    if value == "-0" {
        return false;
    }
    let re = Regex::new(r"^-?[1-9][0-9]*$").expect("invalid regex");
    re.is_match(value)
}

fn is_valid_positive_integer(value: &str) -> bool {
    let re = Regex::new(r"^[1-9][0-9]*$").expect("invalid regex");
    re.is_match(value)
}
