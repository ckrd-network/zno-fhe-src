use std::fmt;
use std::num::ParseIntError;

/// Represents the lifting value `r` in BGV.
///
/// In the BGV encryption scheme of HElib, the parameter `r` is a small non-negative integer
/// that indicates the number of times plaintexts are lifted to a larger ring.
///
/// The choice of `r` influences:
/// - The noise growth.
/// - The computational efficiency.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts values for `r` that can fit within a `u32` type, i.e., between 0 and 4,294,967,295.
///
/// ## Range in HElib:
/// In HElib, `r` is typically a small value, often between 1 and 3. However, the exact range
/// might depend on specific computational contexts and application requirements. Users should
/// refer to HElib's official documentation or relevant publications for detailed guidelines on selecting `r`.
///
/// # Example
///
/// ```
/// # use your_crate_name::R;  // Replace `your_crate_name` with the name of your crate
/// let r = R::new(1).expect("Failed to create R");
/// assert_eq!(r.to_string(), "1");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum R {
    Some(u32),
    Err(ParseIntError),
}

impl R {
    /// Attempts to create an `R` variant from a given u32.
    pub fn new(value: u32) -> Result<Self, ParseIntError> {
        Ok(R::Some(value))
    }
}

/// Provides a default `R` value.
impl Default for R {
    fn default() -> Self {
        R::new(1).unwrap()
    }
}

impl std::str::FromStr for R {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(val) => Ok(R::new(val)),
            Err(e) => Err(e),
        }
    }
}

impl fmt::Display for R {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            R::Some(value) => write!(f, "{}", value),
            R::Err(_) => write!(f, "Error parsing 'r'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_r_value() {
        let r = R::new(1);
        assert!(matches!(r, Ok(R::Some(_))));
    }

    #[test]
    fn test_invalid_r_value() {
        // Given that `r` is defined over all u32 values, there's no specific "invalid" value.
        // This test is kept for structural consistency but won't demonstrate an invalid case.
    }

    #[test]
    fn test_large_r_value() {
        // This test is just to demonstrate that large values of `r` are currently accepted by the type.
        let r = R::new(1_000_000);
        assert!(matches!(r, Ok(R::Some(_))));
    }

    #[test]
    fn test_negative_r_value() {
        let r = R::new(-1 as u32); // Casting negative value to u32
        assert!(matches!(r, Ok(R::Some(_)))); // This is expected to pass because `-1 as u32` results in a valid `u32`.
    }
}
