use std::num::{NonZeroU32, ParseIntError};
use std::fmt;

/// Represents the parameter `c` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `c` typically refers to [specific information about what `c` represents].
/// [Explain the effect of `c` on the security and efficiency of the cryptographic operations, just like with `bits`].
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `c`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `c` often depends on a trade-off between security and performance.
/// [Describe how `c` affects this trade-off]. Users should refer to HElib's official documentation or relevant publications for detailed guidelines on selecting `c`.
///
/// # Example
///
/// ```
/// # use your_crate_name::C;  // Replace `your_crate_name` with the name of your crate
/// let c = C::new(32).expect("Failed to create C");
/// assert_eq!(c.to_string(), "32");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum C {
    Some(NonZeroU32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct CError {
    kind: CErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CErrorKind {
    Zero,
    ParseError(ParseIntError),
}

impl C {
    /// Attempts to create a `C` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, CError> {
        NonZeroU32::new(value)
            .map(C::Some)
            .ok_or_else(|| CError { kind: CErrorKind::Zero })
    }
}

/// Provides a default `C` value.
impl Default for C {
    fn default() -> Self {
        C::new(32).unwrap_or_else(|_| panic!("Default value for C should be valid!"))
    }
}

impl From<ParseIntError> for CError {
    fn from(error: ParseIntError) -> Self {
        CError {
            kind: CErrorKind::ParseError(error),
        }
    }
}

impl core::str::FromStr for C {
    type Err = CError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.parse::<u32>().map_err(CError::from)?;

        NonZeroU32::new(parsed)
            .map(C::Some)
            .ok_or(CError { kind: CErrorKind::Zero })
    }
}

// Implementing the Display trait for C
impl core::fmt::Display for C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            C::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future
        }
    }
}

impl core::fmt::Display for CError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            CErrorKind::Zero => write!(f, "zero is not allowed"),
            CErrorKind::ParseError(e) => e.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_c_value() {
        let c = C::new(32);
        assert!(matches!(c, Ok(C::Some(_))));
    }

    #[test]
    fn test_invalid_c_value() {
        let c = C::new(0);
        assert!(matches!(c, Err(_)));
    }

    #[test]
    fn test_negative_string_c_value() {
        let c = "-1".parse::<C>();
        assert!(matches!(c, Err(_)));
    }
}
