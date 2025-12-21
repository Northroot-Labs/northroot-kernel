use thiserror::Error;

/// Validation errors for canonical primitives.
#[derive(Debug, Error)]
pub enum ValidationError {
    /// When a value does not match the required pattern.
    #[error("{field} ('{value}') is not allowed")]
    PatternMismatch {
        /// Field name that failed validation.
        field: &'static str,
        /// Offending value.
        value: String,
    },
    /// When a numeric quantity exceeds its bounds.
    #[error("{field} ({value}) is out of bounds")]
    OutOfBounds {
        /// Field name that is out of bounds.
        field: &'static str,
        /// Offending value.
        value: String,
    },
}
