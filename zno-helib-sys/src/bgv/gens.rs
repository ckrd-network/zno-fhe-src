use std::num::{NonZeroU32, ParseIntError};
use std::fmt;

/// Represents the `gens` parameter in BGV, which is a vector of integers.
///
/// In the BGV encryption scheme as implemented by HElib, the `gens` parameter specifies the
/// generators for the ring \(\mathbb{Z}/(m\mathbb{Z})\). This parameter aids in defining the structure
/// of the plaintext and ciphertext spaces.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation uses `NonZeroU32` to represent each individual generator value.
/// Each value in the list must fall between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// # Example
///
/// ```
/// # use your_crate_name::Gens;  // Replace `your_crate_name` with the name of your crate
/// let gens = Gens::new(vec![2, 3, 5]).expect("Failed to create Gens");
/// assert_eq!(gens.to_string(), "[2, 3, 5]");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum Gens {
    Some(Vec<NonZeroU32>),
    Err(ParseIntError),
}

impl Gens {
    /// Attempts to create a `Gens` variant from a given vector of u32.
    pub fn new(values: Vec<u32>) -> Result<Self, ParseIntError> {
        let non_zero_values: Result<Vec<NonZeroU32>, _> = values.into_iter()
            .map(NonZeroU32::new)
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| ParseIntError { kind: std::num::IntErrorKind::Zero });

        non_zero_values.map(Gens::Some).or_else(|e| Ok(Gens::Err(e)))
    }
}

impl fmt::Display for Gens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Gens::Some(values) => write!(f, "{:?}", values.iter().map(|v| v.get()).collect::<Vec<_>>()),
            Gens::Err(_) => write!(f, "Error parsing 'gens'"),
        }
    }
}

impl std::str::FromStr for Gens {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Result<Vec<u32>, _> = s.split(',')
            .map(|num_str| num_str.trim().parse::<u32>())
            .collect();
        match values {
            Ok(vec) => Gens::new(vec),
            Err(e) => Ok(Gens::Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_gens_value() {
        let gens = Gens::new(vec![2, 3, 5]);
        assert!(matches!(gens, Ok(Gens::Some(_))));
    }

    #[test]
    fn test_empty_gens_value() {
        let gens = Gens::new(vec![]);
        assert!(matches!(gens, Err(_)));
    }

    #[test]
    fn test_invalid_string_gens_value() {
        let gens = "2, 0, 5".parse::<Gens>();
        assert!(matches!(gens, Ok(Gens::Err(_))));
    }
}
