use std::num::{NonZeroU32, ParseIntError};
use std::fmt;

/// Represents the plaintext space parameter `p` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `p` is chosen as the base of the plaintext space.
/// The choice of `p` affects various properties of the encryption, including:
/// - The size of the plaintext space.
/// - The depth of computable circuits.
/// - Noise behavior.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `p`. Currently, the type
/// uses `NonZeroU32`, which means the range is between 1 and 4,294,967,295 (both inclusive),
/// excluding the value zero.
///
/// ## Range in HElib:
/// The choice of `p` in HElib can vary widely based on the specific application and desired
/// security level. Common choices might be small prime numbers, but more complex selections
/// are possible. Again, users should refer to HElib's official documentation or relevant
/// cryptographic literature for specific guidance on choosing `p`.
///
/// # Example
///
/// ```
/// # use your_crate_name::P;  // Replace `your_crate_name` with the name of your crate
/// let p = P::new(2).expect("Failed to create P");
/// assert_eq!(p.to_string(), "2");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum P {
    Some(NonZeroU32),
    Err(ParseIntError),
}

impl P {
    /// Attempts to create a `P` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, ParseIntError> {
        NonZeroU32::new(value).map_or_else(
            || P::Err(ParseIntError { kind: std::num::IntErrorKind::Zero }),
            P::Some,
        )
    }
}

/// Provides a default `P` value.
impl Default for P {
    fn default() -> Self {
        P::new(2).unwrap_or_else(|_| panic!("Default value for P should be valid!"))
    }
}

impl std::str::FromStr for P {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(val) => Ok(P::new(val)),
            Err(e) => Ok(P::Err(e)),
        }
    }
}

impl fmt::Display for P {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            P::Some(value) => write!(f, "{}", value),
            P::Err(_) => write!(f, "Error parsing 'p'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_p_value() {
        let p = P::new(2);
        assert!(matches!(p, Ok(P::Some(_))));
    }

    #[test]
    fn test_invalid_p_value() {
        let p = P::new(0);
        assert!(matches!(p, Err(_)));
    }

    #[test]
    fn test_out_of_range_p_value() {
        // This test case may need adjustment, as there's no explicit out-of-range behavior defined.
        // But keeping it for symmetry with 'm'.
        let p = P::new(9000);
        assert!(matches!(p, Ok(P::Some(_)))); // Currently, this is expected to pass.
    }

    #[test]
    fn test_negative_p_value() {
        let p = P::new(-1 as u32); // Casting negative value to u32
        assert!(matches!(p, Ok(P::Some(_)))); // This is expected to pass because `-1 as u32` results in a valid `u32`.
    }
}
