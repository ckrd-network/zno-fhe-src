use std::num::{NonZeroU32, ParseIntError};
use std::fmt;

/// Represents the bit-size parameter `bits` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `bits` typically refers to the bit-size of the modulus.
/// The size of this modulus affects the security and efficiency of the cryptographic operations.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `bits`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `bits` often depends on a trade-off between security and performance.
/// Larger bit-sizes generally offer more security but might be less efficient in terms of computation.
/// Users should refer to HElib's official documentation or relevant publications for detailed guidelines on selecting `bits`.
///
/// # Example
///
/// ```
/// # use your_crate_name::Bits;  // Replace `your_crate_name` with the name of your crate
/// let bits = Bits::new(32).expect("Failed to create Bits");
/// assert_eq!(bits.to_string(), "32");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum Bits {
    Some(NonZeroU32),
    Err(ParseIntError),
}

impl Bits {
    /// Attempts to create a `Bits` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, ParseIntError> {
        NonZeroU32::new(value).map_or_else(
            || Bits::Err(ParseIntError { kind: std::num::IntErrorKind::Zero }),
            Bits::Some,
        )
    }
}

/// Provides a default `Bits` value.
impl Default for Bits {
    fn default() -> Self {
        Bits::new(32).unwrap_or_else(|_| panic!("Default value for Bits should be valid!"))
    }
}

impl std::str::FromStr for Bits {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // First, we attempt to parse a u32
        match s.parse::<u32>() {
            Ok(val) => Ok(Bits::new(val)),
            Err(e) => {
                // Check if the error is a string with a negative number
                if s.starts_with('-') {
                    Ok(Bits::Err(ParseIntError { kind: std::num::IntErrorKind::Negative }))
                } else {
                    Ok(Bits::Err(e))
                }
            }
        }
    }
}

impl fmt::Display for Bits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bits::Some(value) => write!(f, "{}", value),
            Bits::Err(_) => write!(f, "Error parsing 'bits'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bits_value() {
        let bits = Bits::new(32);
        assert!(matches!(bits, Ok(Bits::Some(_))));
    }

    #[test]
    fn test_invalid_bits_value() {
        let bits = Bits::new(0);
        assert!(matches!(bits, Err(_)));
    }

    #[test]
    fn test_negative_string_bits_value() {
        let bits = "-1".parse::<Bits>();
        assert!(matches!(bits, Ok(Bits::Err(_))));
    }
}
