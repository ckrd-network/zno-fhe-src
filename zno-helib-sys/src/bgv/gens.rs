use std::num::{NonZeroU32, ParseIntError};
use std::fmt;
use std::str::FromStr;

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
/// # use zno::bgv::Gens;  // Replace `your_crate_name` with the name of your crate
/// let gens = Gens::new(vec![2, 3, 5]).expect("Failed to create Gens");
/// assert_eq!(gens.to_string(), "[2, 3, 5]");
/// ```
///


/// Represents the generators `gens` parameter in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `gens` is a set of generators of the `p^r`-order
/// subgroup, where `p` is the plaintext base and `r` is a positive integer. These generators are essential
/// for the scheme's operations.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `gens`. Each generator is represented
/// as a `NonZeroU32`, providing a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
/// The `gens` parameter itself is a vector of these generators.
///
/// ## Range in HElib:
/// In HElib, the choice of `gens` depends on several factors including the security and performance requirements
/// of the specific use case. Users should refer to HElib's official documentation or relevant publications for
/// detailed guidelines on selecting `gens`.
///
/// # Example
///
/// ```
/// # use your_crate_name::Gens;  // Replace `your_crate_name` with the name of your crate
/// let gens = Gens::new(vec![3, 5, 7]).expect("Failed to create Gens");
/// assert_eq!(gens.to_string(), "3,5,7");
/// ```
///
#[derive(Debug, PartialEq)]
pub struct Gens(Vec<NonZeroU32>);

#[derive(Debug, Clone)]
pub struct GensError {
    kind: GensErrorKind,
}

#[derive(Debug, Clone)]
pub enum GensErrorKind {
    Zero,
    ParseError(ParseIntError),
    // You can add other error kinds if necessary
}

impl Gens {
    /// Attempts to create a `Gens` from a given vector of u32.
    pub fn new(values: Vec<u32>) -> Result<Self, GensError> {
        let mut non_zero_values = Vec::with_capacity(values.len());

        for value in values {
            if let Some(non_zero_value) = NonZeroU32::new(value) {
                non_zero_values.push(non_zero_value);
            } else {
                return Err(GensError { kind: GensErrorKind::Zero });
            }
        }

        Ok(Gens(non_zero_values))
    }
}

/// Provides a default `Gens` value.
impl Default for Gens {
    fn default() -> Self {
        Gens::new(vec![3, 5, 7]).unwrap_or_else(|_| panic!("Default values for Gens should be valid!"))
    }
}

impl From<ParseIntError> for GensError {
    fn from(error: ParseIntError) -> Self {
        GensError {
            kind: GensErrorKind::ParseError(error),
        }
    }
}

impl FromStr for Gens {
    type Err = GensError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Result<Vec<_>, _> = s.split(',')
            .map(|part| part.parse::<u32>().map_err(GensError::from))
            .collect();
        let parts = parts?;

        Gens::new(parts)
    }
}

// Implementing the Display trait for Gens
impl fmt::Display for Gens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let gens_as_strings: Vec<_> = self.0.iter().map(ToString::to_string).collect();
        write!(f, "{}", gens_as_strings.join(","))
    }
}

impl fmt::Display for GensError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            GensErrorKind::Zero => write!(f, "zero is not allowed"),
            GensErrorKind::ParseError(e) => e.fmt(f),
            // Handle other kinds of errors if they are added in the future
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_gens_values() {
        let gens = Gens::new(vec![3, 5, 7]);
        assert!(gens.is_ok());
    }

    #[test]
    fn test_invalid_gens_values() {
        let gens = Gens::new(vec![0, 5, 7]);
        assert!(matches!(gens, Err(_)));
    }

    #[test]
    fn test_string_parsing() {
        let gens: Result<Gens, _> = "3,5,7".parse();
        assert!(gens.is_ok());
    }

    #[test]
    fn test_invalid_string_parsing() {
        let gens: Result<Gens, _> = "3,-5,7".parse();
        assert!(matches!(gens, Err(_)));
    }
}
