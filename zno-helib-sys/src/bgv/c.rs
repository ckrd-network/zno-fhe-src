use std::num::{NonZeroU32, ParseIntError};
use std::fmt;

/// Represents the ciphertext levels parameter `c` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `c` determines the number of levels in the leveled encryption.
/// It affects the depth of computations that can be supported without bootstrapping.
/// Higher values for `c` allow for deeper circuits/computations at the expense of larger parameter sizes and slower operations.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `c`. Currently, the type
/// uses `NonZeroU32`, which means the range is between 1 and 4,294,967,295 (both inclusive),
/// excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `c` typically depends on the desired depth of the computation. Small values like 2 or 3 are common for simpler tasks,
/// but it can be larger for deeper computations. Users should refer to HElib's official documentation
/// or relevant publications for detailed guidelines on selecting `c`.
///
/// # Example
///
/// ```
/// # use your_crate_name::C;  // Replace `your_crate_name` with the name of your crate
/// let c = C::new(3).expect("Failed to create C");
/// assert_eq!(c.to_string(), "3");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum C {
    Some(NonZeroU32),
    Err(ParseIntError),
}

impl C {
    /// Attempts to create a `C` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, ParseIntError> {
        NonZeroU32::new(value).map_or_else(
            || C::Err(ParseIntError { kind: std::num::IntErrorKind::Zero }),
            C::Some,
        )
    }
}

/// Provides a default `C` value.
impl Default for C {
    fn default() -> Self {
        C::new(2).unwrap_or_else(|_| panic!("Default value for C should be valid!"))
    }
}

impl std::str::FromStr for C {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(val) => Ok(C::new(val)),
            Err(e) => Ok(C::Err(e)),
        }
    }
}

impl fmt::Display for C {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            C::Some(value) => write!(f, "{}", value),
            C::Err(_) => write!(f, "Error parsing 'c'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_c_value() {
        let c = C::new(3);
        assert!(matches!(c, Ok(C::Some(_))));
    }

    #[test]
    fn test_invalid_c_value() {
        let c = C::new(0);
        assert!(matches!(c, Err(_)));
    }

    // For the parameter c, out-of-range tests might not be as relevant since the definition
    // itself doesn't suggest any common "upper limit". However, you can implement them as needed.
}
