use std::num::{NonZeroU32, ParseIntError};
use std::fmt;

/// Represents the cyclotomic polynomial parameter `m` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `m` is chosen to define a cyclotomic polynomial Phi_m(X).
/// This parameter greatly affects the noise and efficiency of the system.
/// Commonly, `m` is chosen such that Phi_m(X) has a large number of slots,
/// which allows for batching of operations.
///
/// The choice of `m` is often influenced by:
/// - The need to support certain operations (e.g., rotations).
/// - The desire to have a large plaintext space.
/// - Performance considerations.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `m`. Currently, the type
/// uses `NonZeroU32`, which means the range is between 1 and 4,294,967,295 (both inclusive),
/// excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `m` is generally more flexible, but for practical efficiency and
/// security reasons, it's commonly chosen from a set of values that have specific properties.
/// Often, values like 2047, 4095, or 8191 are used as they strike a balance between performance
/// and security. However, HElib might support much larger values depending on the computational
/// context, hardware, and application specifics. Users should refer to HElib's official
/// documentation or relevant publications for detailed guidelines on selecting `m`.
///
/// # Example
///
/// ```
/// # use your_crate_name::M;  // Replace `your_crate_name` with the name of your crate
/// let m = M::new(4095).expect("Failed to create M");
/// assert_eq!(m.to_string(), "4095");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum M {
    Some(NonZeroU32),
    Err(ParseIntError),
}

impl M {
    /// Attempts to create an `M` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, ParseIntError> {
        NonZeroU32::new(val).map_or_else(
            || M::Err(ParseIntError { kind: std::num::IntErrorKind::Zero }),
            M::Some,
        )
    }
}
/// Provides a default `M` value.
impl Default for M {
    fn default() -> Self {
        M::new(4095).unwrap_or_else(|_| panic!("Default value for M should be valid!"))
    }
}

impl std::str::FromStr for M {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(val) => Ok(M::new(val)),
            Err(e) => Ok(M::Err(e)),
        }
    }
}

impl fmt::Display for M {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            M::Some(value) => {
                // In BGV, 'm' is the cyclotomic order, which is typically a product of small primes.
                // Displaying 'm' directly as its integer value.
                write!(f, "{}", value)
            },
            M::Err(_) => write!(f, "Error parsing 'm'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_m_value() {
        let m = M::new(4095);
        assert!(matches!(m, Ok(M::Some(_))));
    }

    #[test]
    fn test_invalid_m_value() {
        let m = M::new(0);
        assert!(matches!(m, Err(_)));
    }

    #[test]
    fn test_out_of_range_m_value() {
        let m = M::new(9000);
        assert!(matches!(m, Err(_)));
    }

    #[test]
    fn test_negative_m_value() {
        let m = M::new(-1 as u32); // Casting negative value to u32
        assert!(matches!(m, Err(_)));
    }
}