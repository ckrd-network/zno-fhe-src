use std::num::{NonZeroU32, ParseIntError};
use std::fmt;

/// Represents the cyclotomic order parameter `m` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `m` determines the cyclotomic order, which is crucial for defining the polynomial ring over which computations are performed.
/// The choice of `m` affects the security and efficiency of the cryptographic operations.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `m`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `m` is significant as it influences the security level and the efficiency of operations. Larger values of `m` provide better security but increase computational requirements.
/// Users should refer to HElib's official documentation or relevant cryptographic literature for detailed guidelines on selecting `m`.
///
/// # Example
///
/// ```
/// # use your_crate_name::M;  // Replace `your_crate_name` with the name of your crate
/// let m = M::new(32).expect("Failed to create M");
/// assert_eq!(m.to_string(), "32");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum M {
    Some(NonZeroU32),
    // More variants can be added here in the future.
}

#[derive(Debug, Clone)]
pub struct MError {
    kind: MErrorKind,
}

#[derive(Debug, Clone)]
pub enum MErrorKind {
    Zero,
    ParseError(ParseIntError),
    // other kinds of errors can be added here
}

impl M {
    /// Attempts to create an `M` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, MError> {
        NonZeroU32::new(value)
            .map(M::Some)
            .ok_or_else(|| MError { kind: MErrorKind::Zero })
    }
}

/// Provides a default `M` value.
impl Default for M {
    fn default() -> Self {
        M::new(32).unwrap_or_else(|_| panic!("Default value for M should be valid!"))
    }
}

impl From<ParseIntError> for MError {
    fn from(error: ParseIntError) -> Self {
        MError {
            kind: MErrorKind::ParseError(error),
        }
    }
}

impl core::str::FromStr for M {
    type Err = MError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.parse::<u32>().map_err(MError::from)?;

        NonZeroU32::new(parsed)
            .map(M::Some)
            .ok_or(MError { kind: MErrorKind::Zero })
    }
}

// Implementing the Display trait for M
impl core::fmt::Display for M {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            M::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future
        }
    }
}

impl core::fmt::Display for MError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            MErrorKind::Zero => write!(f, "zero is not allowed"),
            MErrorKind::ParseError(e) => e.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_m_value() {
        let m = M::new(32);
        assert!(matches!(m, Ok(M::Some(_))));
    }

    #[test]
    fn test_invalid_m_value() {
        let m = M::new(0);
        assert!(matches!(m, Err(_)));
    }

    #[test]
    fn test_negative_string_m_value() {
        let m = "-1".parse::<M>();
        assert!(matches!(m, Ok(M::Err(_)))); // This line will need to be adjusted based on your error handling strategy.
    }
}
