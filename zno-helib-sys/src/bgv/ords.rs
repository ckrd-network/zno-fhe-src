use core::str::FromStr;
use std::fmt;
use std::num::{NonZeroU32, ParseIntError};

/// Represents the `ords` parameter in BGV, which is a vector of integers.
///
/// In the BGV encryption scheme as implemented by HElib, the `ords` parameter specifies the
/// orders of the generators that are defined by the `gens` parameter. These orders and their
/// associated generators are fundamental in creating the ring structure that underpins the
/// encryption scheme.
///
/// Conceptually, if `g` is a generator from the `gens` vector, then its associated order from the
/// `ords` vector indicates that `g` raised to that order is congruent to 1 mod `m`. For negative
/// orders, it further signifies that the generator `g` has an order that divides the absolute value
/// of the negative order, but does not divide half of that value.
///
/// The `ords` vector must have the same length as the `gens` vector. Each element of `ords`
/// provides additional information on the structure of the plaintext space and influences
/// the efficiency of certain operations like encoding, encryption, and homomorphic transformations.
///
/// # Example
///
/// ```
/// # use your_crate_name::Ords;  // Replace `your_crate_name` with the name of your crate
/// let ords = Ords::new(vec![2, -3, 5]).expect("Failed to create Ords");
/// assert_eq!(ords.to_string(), "[2, -3, 5]");
/// ```
///


/// Represents the `ords` parameter in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `ords` typically refers to the orders of the small
/// prime factors of the modulus chain. Each element in `ords` is a `NonZeroU32`, representing the order
/// of a corresponding small prime.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for each element in `ords`. Currently, the type
/// uses `NonZeroU32` for each element. This provides a range between 1 and 4,294,967,295 (both inclusive),
/// excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `ords` contributes to the construction of the modulus chain and affects
/// the security and efficiency of cryptographic operations. A more detailed explanation and guidelines
/// can be found in HElib's official documentation and relevant cryptographic publications.
///
/// # Example
///
/// ```
/// # use your_crate_name::Ords; // Replace `your_crate_name` with the name of your crate
/// let ords = Ords::new(vec![2, 3, 5]).expect("Failed to create Ords");
/// assert_eq!(ords.to_string(), "[2, 3, 5]");
/// ```
///
#[derive(Debug, PartialEq)]
pub struct Ords {
    values: Vec<NonZeroU32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrdsError {
    kind: OrdsErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OrdsErrorKind {
    Zero,
    ParseError(ParseIntError),
}

impl Ords {
    /// Attempts to create an `Ords` from a given vector of u32.
    pub fn new(values: Vec<u32>) -> Result<Self, OrdsError> {
        let mut non_zero_values = Vec::with_capacity(values.len());

        for value in values {
            if let Some(non_zero) = NonZeroU32::new(value) {
                non_zero_values.push(non_zero);
            } else {
                return Err(OrdsError { kind: OrdsErrorKind::Zero });
            }
        }

        Ok(Ords { values: non_zero_values })
    }
}

/// Provides a default `Ords` value.
impl Default for Ords {
    fn default() -> Self {
        Ords::new(vec![6, 4, 6]).unwrap_or_else(|_| panic!("Default value for Ords should be valid!"))
    }
}

impl From<ParseIntError> for OrdsError {
    fn from(error: ParseIntError) -> Self {
        OrdsError {
            kind: OrdsErrorKind::ParseError(error),
        }
    }
}

// Implementing the FromStr trait for Ords, assuming a comma-separated list of numbers in a string
impl FromStr for Ords {
    type Err = OrdsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim_matches(|p| p == '[' || p == ']').split(',').collect();
        let mut values = Vec::with_capacity(parts.len());

        for part in parts {
            let value: u32 = part.trim().parse().map_err(OrdsError::from)?;
            if let Some(non_zero) = NonZeroU32::new(value) {
                values.push(non_zero);
            } else {
                return Err(OrdsError { kind: OrdsErrorKind::Zero });
            }
        }

        Ok(Ords { values })
    }
}

// Implementing the Display trait for Ords
impl core::fmt::Display for Ords {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let as_strings: Vec<String> = self.values.iter().map(|n| n.get().to_string()).collect();
        write!(f, "[{}]", as_strings.join(", "))
    }
}

impl core::fmt::Display for OrdsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            OrdsErrorKind::Zero => write!(f, "zero is not allowed in ords"),
            OrdsErrorKind::ParseError(e) => e.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ords_values() {
        let ords = Ords::new(vec![2, 3, 5]);
        assert!(ords.is_ok());
    }

    #[test]
    fn test_invalid_ords_values() {
        let ords = Ords::new(vec![0, 3, 5]);
        assert!(matches!(ords, Err(_)));
    }

    #[test]
    fn test_string_parsing() {
        let ords: Result<Ords, _> = "[2, 3, 5]".parse();
        assert!(ords.is_ok());
    }

    #[test]
    fn test_negative_string_parsing() {
        let ords: Result<Ords, _> = "[-1, 2, 3]".parse();
        assert!(ords.is_err());
    }
}
