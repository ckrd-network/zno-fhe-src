use std::num::{NonZeroU32, ParseIntError};
use std::fmt;

/// Represents the plaintext modulus parameter `p` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `p` defines the plaintext modulus.
/// This parameter is crucial in defining the structure of the plaintext space and affects
/// the efficiency and security of the encryption scheme.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `p`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `p` is a significant factor in the setup of the encryption scheme.
/// It plays a crucial role in the noise growth during computations and thereby affects the
/// overall depth of computation and security. Users should refer to HElib's official documentation
/// or relevant academic publications for detailed guidelines on selecting `p`.
///
/// # Example
///
/// ```
/// # use your_crate_name::P;  // Replace `your_crate_name` with the name of your crate
/// let p = P::new(65537).expect("Failed to create P");
/// assert_eq!(p.to_string(), "65537");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum P {
    Some(NonZeroU32),
    // Add other variants if needed in the future
}

#[derive(Debug, Clone)]
pub struct PError {
    kind: PErrorKind,
}

#[derive(Debug, Clone)]
pub enum PErrorKind {
    Zero,
    ParseError(ParseIntError),
}

impl P {
    /// Attempts to create a `P` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, PError> {
        NonZeroU32::new(value)
            .map(P::Some)
            .ok_or_else(|| PError { kind: PErrorKind::Zero })
    }
}

/// Provides a default `P` value.
impl Default for P {
    fn default() -> Self {
        P::new(65537).unwrap_or_else(|_| panic!("Default value for P should be valid!"))
    }
}

impl From<ParseIntError> for PError {
    fn from(error: ParseIntError) -> Self {
        PError {
            kind: PErrorKind::ParseError(error),
        }
    }
}

impl core::str::FromStr for P {
    type Err = PError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.parse::<u32>().map_err(PError::from)?;

        NonZeroU32::new(parsed)
            .map(P::Some)
            .ok_or(PError { kind: PErrorKind::Zero })
    }
}

// Implementing the Display trait for P
impl core::fmt::Display for P {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            P::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future
        }
    }
}

impl core::fmt::Display for PError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            PErrorKind::Zero => write!(f, "zero is not allowed"),
            PErrorKind::ParseError(e) => e.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_p_value() {
        let p = P::new(65537);
        assert!(matches!(p, Ok(P::Some(_))));
    }

    #[test]
    fn test_invalid_p_value() {
        let p = P::new(0);
        assert!(matches!(p, Err(_)));
    }

    #[test]
    fn test_negative_string_p_value() {
        let p = "-1".parse::<P>();
        assert!(matches!(p, Ok(P::Err(_))));
    }
}
