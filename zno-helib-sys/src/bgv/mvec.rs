use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;

/// Represents the `mvec` parameter in BGV, which is a vector of integers.
///
/// In the BGV encryption scheme as implemented by HElib, the `mvec` parameter specifies a
/// vector of moduli. The bootstrapping procedure in BGV often requires defining an encrypted
/// value modulo multiple `m` values, thus leading to the introduction of the `mvec` parameter.
///
/// Bootstrapping is an operation that reduces noise in the ciphertext, allowing more homomorphic
/// operations on the data. The `mvec` vector plays a pivotal role in this process.
///
/// This FFI representation for `mvec` accepts integers as its elements. Depending on the context
/// and the exact details of the BGV implementation in HElib, these might represent different
/// moduli that the ciphertexts can be defined over.
///
/// # Example
///
/// ```
/// # use your_crate_name::Mvec;  // Replace `your_crate_name` with the name of your crate
/// let mvec = Mvec::new(vec![2, 3, 5]).expect("Failed to create Mvec");
/// assert_eq!(mvec.to_string(), "[2, 3, 5]");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum Mvec {
    Some(Vec<u32>),
    Err(ParseIntError),
}

impl Mvec {
    /// Attempts to create an `Mvec` variant from a given vector of u32.
    pub fn new(values: Vec<u32>) -> Result<Self, ParseIntError> {
        // No zero check here, as we're assuming all values are valid for mvec. If this isn't the case, validation can be added.
        Ok(Mvec::Some(values))
    }
}

impl fmt::Display for Mvec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mvec::Some(values) => write!(f, "{:?}", values),
            Mvec::Err(_) => write!(f, "Error parsing 'mvec'"),
        }
    }
}

impl FromStr for Mvec {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Result<Vec<u32>, _> = s.split(',')
            .map(|num_str| num_str.trim().parse::<u32>())
            .collect();
        match values {
            Ok(vec) => Mvec::new(vec),
            Err(e) => Ok(Mvec::Err(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mvec_value() {
        let mvec = Mvec::new(vec![2, 3, 5]);
        assert!(matches!(mvec, Ok(Mvec::Some(_))));
    }

    #[test]
    fn test_empty_mvec_value() {
        let mvec = Mvec::new(vec![]);
        assert!(matches!(mvec, Ok(Mvec::Some(_)))); // Assuming empty `mvec` is valid.
    }

    #[test]
    fn test_invalid_string_mvec_value() {
        let mvec = "2, -, 5".parse::<Mvec>();
        assert!(matches!(mvec, Ok(Mvec::Err(_))));
    }
}
