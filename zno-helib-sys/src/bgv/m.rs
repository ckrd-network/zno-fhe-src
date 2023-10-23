use std::num::ParseIntError;
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
    Some(core::num::NonZeroU32),
    // More variants can be added here in the future.
}

#[derive(Debug, Clone, PartialEq)]
pub struct MError {
    pub kind: MErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MErrorKind {
    OutOfRange(String),
    ParseError(ParseIntError),
    Zero,
    Generic(String),// other kinds of errors can be added here
}

impl M {
    /// Create an `M` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, MError> {
        core::num::NonZeroU32::new(value)
            .map(M::Some)
            .ok_or_else(|| MError { kind: MErrorKind::Zero })
    }

    /// Create an `M` variant from a given i64.
    pub fn from_i64(value: i64) -> Result<Self, MError> {
        if (1..=(u32::MAX as i64)).contains(&value) {
            // Direct conversion as the range has been checked
            M::new(value as u32)
        } else if value == 0 {
            Err(MError { kind: MErrorKind::Zero })
        } else {
            Err(MError { kind: MErrorKind::OutOfRange("Value must be in the range of 1 to u32::MAX".into()) })
        }
    }
}

impl crate::bgv::ToU32<MError> for M {
    fn to_u32(&self) -> Result<u32, MError> {
        match self {
            M::Some(non_zero_u32) => Ok(non_zero_u32.get()),
            _ => Err(MError {
                kind: MErrorKind::OutOfRange("Value out of range of u32".into()), // A generic error.
            }),
        }
    }
}

/// Provides a default `M` value. A panic should never occur, as this is a safer default.
impl Default for M {
    fn default() -> Self {
        M::Some(core::num::NonZeroU32::new(32).expect("32 is a valid non-zero u32 value."))
    }
}

impl std::error::Error for MError {}

// Implement From for each error type to convert into MError
impl From<std::io::Error> for MError {
    fn from(e: std::io::Error) -> MError {
        MError {
            kind: MErrorKind::Generic(e.to_string()),
        }
    }
}

impl From<std::num::ParseIntError> for MError {
    fn from(error: std::num::ParseIntError) -> Self {
        MError {
            kind: MErrorKind::ParseError(error),
        }
    }
}

impl core::str::FromStr for M {
    type Err = MError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(value) => M::new(value),
            Err(_) => {
                // If parsing as u32 failed, try parsing as u64 to determine if it's a range issue
                match s.parse::<u64>() {
                    Ok(value) => {
                        if value > u32::MAX as u64 {
                            Err(MError { kind: MErrorKind::OutOfRange("Value out of range for u32".to_string()) })
                        } else {
                            // This branch implies logical error: the value fits within u32, but parse::<u32>() failed.
                            // It should not actually happen in normal circumstances if the input is a valid number.
                            Err(MError { kind: MErrorKind::Generic("Invalid number format".to_string()) })
                        }
                    },
                    Err(_) => {
                        // If parsing as u64 also failed, then the string does not represent a valid number.
                        Err(MError { kind: MErrorKind::ParseError(s.parse::<u32>().unwrap_err()) })
                    }
                }
            }
        }
    }
}

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
            MErrorKind::OutOfRange(s) => write!(f, "{}", s),
            MErrorKind::ParseError(e) => e.fmt(f),
            MErrorKind::Zero => write!(f, "zero is not allowed"),
            MErrorKind::Generic(g) => g.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_m_valid_value() {
        let m = M::new(32);
        assert!(matches!(m, Ok(M::Some(_))));
    }

    #[test]
    fn test_m_new_zero() {
        // Trying to create M with zero should yield a Zero error.
        let m = M::new(0);
        assert_eq!(m, Err(MError { kind: MErrorKind::Zero }));
    }

    #[test]
    fn test_m_invalid_value() {
        let m = M::new(0);
        assert!(matches!(m, Err(_)));
    }

    #[test]
    fn test_m_from_str_valid() {
        // Parsing a valid value ("1") should succeed.
        let m = "1".parse::<M>();
        assert!(m.is_ok());
        assert_eq!(m.unwrap(), M::Some(core::num::NonZeroU32::new(1).unwrap()));
    }

        #[test]
    fn test_m_from_str_zero() {
        // Trying to parse "0" into M should yield a Zero error.
        let m = "0".parse::<M>();
        assert_eq!(m, Err(MError { kind: MErrorKind::Zero }));
    }

    #[test]
    fn test_m_from_str_out_of_range() {
        let result = (u64::MAX).to_string().parse::<M>();
        assert!(matches!(result, Err(MError { kind: MErrorKind::OutOfRange(_) })));
    }

    #[test]
    fn test_m_from_str_invalid() {
        // Parsing an invalid value ("-1") should fail.
        let m = "-1".parse::<M>();
        assert!(m.is_err());
        // You can also check for a specific error if you want
        match m {
            Err(MError { kind: MErrorKind::ParseError(_) }) => (), // This is expected, do nothing
            _ => panic!("Expected ParseError"), // Panic with a custom message in all other cases
        }
    }

    #[test]
    fn test_m_new() {
        assert!(matches!(M::new(1), Ok(M::Some(_))));
        assert_eq!(M::new(0), Err(MError { kind: MErrorKind::Zero }));
    }

    #[test]
    fn test_m_from_i64_zero() {
        assert!(matches!(M::from_i64(1), Ok(M::Some(_))));
        assert_eq!(M::from_i64(0), Err(MError { kind: MErrorKind::Zero }));
    }

    #[test]
    fn test_m_from_str_non_numeric() {
        let result = "non_numeric".parse::<M>();
        assert!(matches!(result, Err(MError { kind: MErrorKind::ParseError(_) })));
    }

    #[test]
    fn test_m_default() {
        assert!(matches!(M::default(), M::Some(_)));
    }
}
