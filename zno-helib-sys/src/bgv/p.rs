use core::num::{NonZeroU32, ParseIntError};
use std::{fmt, io, str};

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
    // More variants can be added here in the future.
}

#[derive(Debug, Clone, PartialEq)]
pub struct PError {
    pub kind: PErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PErrorKind {
    OutOfRange(String),
    ParseError(ParseIntError),
    Zero,
    Generic(String), // other kinds of errors can be added here
}

impl P {
    /// Create a `P` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, PError> {
        NonZeroU32::new(value)
            .map(P::Some)
            .ok_or_else(|| PError { kind: PErrorKind::Zero })
    }

    /// Create a `P` variant from a given i64.
    pub fn from_i64(value: i64) -> Result<Self, PError> {
        if (1..=(u32::MAX as i64)).contains(&value) {
            // Direct conversion as the range has been checked
            P::new(value as u32)
        } else if value == 0 {
            Err(PError { kind: PErrorKind::Zero })
        } else {
            Err(PError { kind: PErrorKind::OutOfRange("Value must be in the range of 1 to u32::MAX".into()) })
        }
    }
}

// Assuming there's a trait ToU32 in the bgv module for conversion purposes.
impl crate::bgv::ToU32<PError> for P {
    fn to_u32(&self) -> Result<u32, PError> {
        match self {
            P::Some(non_zero_u32) => Ok(non_zero_u32.get()),
            // For other variants, return an appropriate error.
            _ => Err(PError {
                kind: PErrorKind::OutOfRange("Value out of range of u32".into()), // A generic error.
            }),
        }
    }
}

/// Provides a default `P` value. A panic should never occur, as this is a safer default.
impl Default for P {
    fn default() -> Self {
        P::Some(NonZeroU32::new(11).expect("11 is a valid non-zero u32 value.")) // Assuming 11 is a safe default for P.
    }
}

impl std::error::Error for PError {}

// Implement From for each error type to convert into PError
impl From<io::Error> for PError {
    fn from(e: io::Error) -> PError {
        PError {
            kind: PErrorKind::Generic(e.to_string()),
        }
    }
}

impl From<ParseIntError> for PError {
    fn from(error: ParseIntError) -> Self {
        PError {
            kind: PErrorKind::ParseError(error),
        }
    }
}

impl str::FromStr for P {
    type Err = PError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(value) => P::new(value),
            Err(_) => {
                // If parsing as u32 failed, try parsing as u64 to determine if it's a range issue
                match s.parse::<u64>() {
                    Ok(value) => {
                        if value > u32::MAX as u64 {
                            Err(PError { kind: PErrorKind::OutOfRange("Value out of range for u32".to_string()) })
                        } else {
                            // This branch implies a logical error: the value fits within u32, but parse::<u32>() failed.
                            Err(PError { kind: PErrorKind::Generic("Invalid number format".to_string()) })
                        }
                    },
                    Err(_) => {
                        // If parsing as u64 also failed, then the string does not represent a valid number.
                        Err(PError { kind: PErrorKind::ParseError(s.parse::<u32>().unwrap_err()) })
                    }
                }
            }
        }
    }
}

impl core::fmt::Display for P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            P::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future.
        }
    }
}

impl core::fmt::Display for PError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            PErrorKind::OutOfRange(s) => write!(f, "{}", s),
            PErrorKind::ParseError(e) => e.fmt(f),
            PErrorKind::Zero => write!(f, "zero is not allowed"),
            PErrorKind::Generic(g) => g.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p_valid_value() {
        let p = P::new(32);
        assert!(matches!(p, Ok(P::Some(_))));
    }

    #[test]
    fn test_p_new_zero() {
        // Trying to create P with zero should yield a Zero error.
        let p = P::new(0);
        assert_eq!(p, Err(PError { kind: PErrorKind::Zero }));
    }

    #[test]
    fn test_p_invalid_value() {
        let p = P::new(0);
        assert!(matches!(p, Err(_)));
    }

    #[test]
    fn test_p_from_str_valid() {
        // Parsing a valid value ("1") should succeed.
        let p = "1".parse::<P>();
        assert!(p.is_ok());
        assert_eq!(p.unwrap(), P::Some(core::num::NonZeroU32::new(1).unwrap()));
    }

    #[test]
    fn test_p_from_str_zero() {
        // Trying to parse "0" into P should yield a Zero error.
        let p = "0".parse::<P>();
        assert_eq!(p, Err(PError { kind: PErrorKind::Zero }));
    }

    #[test]
    fn test_p_from_str_out_of_range() {
        let result = (u64::MAX).to_string().parse::<P>();
        assert!(matches!(result, Err(PError { kind: PErrorKind::OutOfRange(_) })));
    }

    #[test]
    fn test_p_from_str_invalid() {
        // Parsing an invalid value ("-1") should fail.
        let p = "-1".parse::<P>();
        assert!(p.is_err());
        // You can also check for a specific error if you want
        match p {
            Err(PError { kind: PErrorKind::ParseError(_) }) => (), // This is expected, do nothing
            _ => panic!("Expected ParseError"), // Panic with a custom message in all other cases
        }
    }

    #[test]
    fn test_p_new() {
        assert!(matches!(P::new(1), Ok(P::Some(_))));
        assert_eq!(P::new(0), Err(PError { kind: PErrorKind::Zero }));
    }

    #[test]
    fn test_p_from_i64_zero() {
        assert!(matches!(P::from_i64(1), Ok(P::Some(_))));
        assert_eq!(P::from_i64(0), Err(PError { kind: PErrorKind::Zero }));
    }

    #[test]
    fn test_p_from_str_non_numeric() {
        let result = "non_numeric".parse::<P>();
        assert!(matches!(result, Err(PError { kind: PErrorKind::ParseError(_) })));
    }

    #[test]
    fn test_p_default() {
        assert!(matches!(P::default(), P::Some(_)));
    }
}
