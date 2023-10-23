use std::num::{NonZeroU32, ParseIntError};
use std::fmt;

/// Represents the Hensel lifting degree `r` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `r` typically refers to the Hensel lifting degree.
/// This parameter affects the encoding of integers into polynomials and plays a crucial role in
/// the efficiency of operations and the capacity of the ciphertext.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `r`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `r` often depends on the desired balance between the complexity of operations
/// and the noise growth in ciphertexts. Users should refer to HElib's official documentation or relevant
/// publications for detailed guidelines on selecting `r`.
///
/// # Example
///
/// ```
/// # use your_crate_name::R;  // Replace `your_crate_name` with the name of your crate
/// let r = R::new(32).expect("Failed to create R");
/// assert_eq!(r.to_string(), "32");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum R {
    Some(NonZeroU32),
    // Add other variants if necessary
}

#[derive(Debug, Clone)]
pub struct RError {
    kind: RErrorKind,
}

#[derive(Debug, Clone)]
pub enum RErrorKind {
    Zero,
    ParseError(ParseIntError),
}

impl R {
    /// Attempts to create a `R` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, RError> {
        NonZeroU32::new(value)
            .map(R::Some)
            .ok_or_else(|| RError { kind: RErrorKind::Zero })
    }
}

/// Provides a default `R` value.
impl Default for R {
    fn default() -> Self {
        R::new(1).unwrap_or_else(|_| panic!("Default value for R should be valid!"))
    }
}

impl From<ParseIntError> for RError {
    fn from(error: ParseIntError) -> Self {
        RError {
            kind: RErrorKind::ParseError(error),
        }
    }
}

impl core::str::FromStr for R {
    type Err = RError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.parse::<u32>().map_err(RError::from)?;

        NonZeroU32::new(parsed)
            .map(R::Some)
            .ok_or(RError { kind: RErrorKind::Zero })
    }
}

// Implementing the Display trait for R
impl core::fmt::Display for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            R::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future
        }
    }
}

impl core::fmt::Display for RError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            RErrorKind::Zero => write!(f, "zero is not allowed"),
            RErrorKind::ParseError(e) => e.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_lifting_degree_value() {
        let r = R::new(32);
        assert!(matches!(r, Ok(R::Some(_))));
    }

    #[test]
    fn test_invalid_lifting_degree_value() {
        let r = R::new(0);
        assert!(matches!(r, Err(_)));
    }

    #[test]
    fn test_negative_string_lifting_degree_value() {
        let r = "-1".parse::<R>();
        assert!(matches!(r, Err(_)));  // Should not match Ok since negative values are not allowed
    }
}
