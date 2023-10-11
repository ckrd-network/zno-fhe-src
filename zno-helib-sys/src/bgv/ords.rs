use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

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
#[derive(Debug, PartialEq)]
pub enum Ords {
    Some(Vec<i32>),
    Err(ParseIntError),
}

impl Ords {
    /// Attempts to create an `Ords` variant from a given vector of i32.
    pub fn new(values: Vec<i32>) -> Result<Self, ParseIntError> {
        // Here, we may want to introduce additional checks or validations based on the behavior of the FFI.
        Ok(Ords::Some(values))
    }
}

impl fmt::Display for Ords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Ords::Some(values) => write!(f, "{:?}", values),
            Ords::Err(_) => write!(f, "Error parsing 'ords'"),
        }
    }
}

impl FromStr for Ords {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Result<Vec<i32>, _> = s.split(',')
            .map(|num_str| num_str.trim().parse::<i32>())
            .collect();
        match values {
            Ok(vec) => Ords::new(vec),
            Err(e) => Ok(Ords::Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ords_value() {
        let ords = Ords::new(vec![2, -3, 5]);
        assert!(matches!(ords, Ok(Ords::Some(_))));
    }

    #[test]
    fn test_empty_ords_value() {
        let ords = Ords::new(vec![]);
        assert!(matches!(ords, Ok(Ords::Some(_))));  // An empty `ords` might be valid based on the FFI behavior.
    }

    #[test]
    fn test_invalid_string_ords_value() {
        let ords = "2, -, 5".parse::<Ords>();
        assert!(matches!(ords, Ok(Ords::Err(_))));
    }
}
