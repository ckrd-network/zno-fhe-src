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
}

#[derive(Debug, Clone, PartialEq)]
pub struct BitsError {
    kind: BitsErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BitsErrorKind {
    Zero,
    ParseError(ParseIntError),
}

impl Bits {
    /// Attempts to create a `Bits` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, BitsError> {
        NonZeroU32::new(value)
            .map(Bits::Some)
            .ok_or_else(|| BitsError { kind: BitsErrorKind::Zero })
    }
}

/// Provides a default `Bits` value.
impl Default for Bits {
    fn default() -> Self {
        Bits::new(32).unwrap_or_else(|_| panic!("Default value for Bits should be valid!"))
    }
}

impl From<ParseIntError> for BitsError {
    fn from(error: ParseIntError) -> Self {
        BitsError {
            kind: BitsErrorKind::ParseError(error),
        }
    }
}

impl core::str::FromStr for Bits {
    type Err = BitsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = s.parse::<u32>().map_err(BitsError::from)?;

        NonZeroU32::new(parsed)
            .map(Bits::Some)
            .ok_or(BitsError { kind: BitsErrorKind::Zero })
    }
}

// Implementing the Display trait for Bits
impl core::fmt::Display for Bits {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Bits::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future
        }
    }
}

impl core::fmt::Display for BitsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            BitsErrorKind::Zero => write!(f, "zero is not allowed"),
            BitsErrorKind::ParseError(e) => e.fmt(f),
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

    // #[test]
    // fn test_negative_string_bits_value() {
    //     let bits = "-1".parse::<Bits>();
    //     assert!(matches!(bits, Ok(Bits::Err(_))));
    // }
}
