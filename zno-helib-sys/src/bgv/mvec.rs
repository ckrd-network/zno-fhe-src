use std::num::{NonZeroU32, ParseIntError};
use std::fmt;
use std::str::FromStr;

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


/// Represents the `mvec` parameter in BGV, a vector of positive integers.
///
/// In HElib's BGV encryption scheme, `mvec` is a parameter that holds a vector of integers
/// representing the cyclotomic indices. Each element in `mvec` affects the structure of the
/// ciphertexts and the efficiency of the cryptographic operations.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation requires that each element of `mvec` be a positive integer,
/// hence the use of `NonZeroU32`. The elements must be within the range of 1 to 4,294,967,295 (both inclusive).
///
/// ## Range in HElib:
/// The choice of values for `mvec` in HElib impacts both the security and performance of operations.
/// Specific values determine the algebraic structure used for the ciphertexts. Users should refer to
/// HElib's official documentation or relevant publications for detailed guidelines on selecting `mvec`.
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
pub struct Mvec {
    values: Vec<NonZeroU32>,
}

#[derive(Debug, Clone)]
pub struct MvecError {
    kind: MvecErrorKind,
}

#[derive(Debug, Clone)]
pub enum MvecErrorKind {
    ZeroValue,
    ParseError(ParseIntError),
}

impl Mvec {
    /// Attempts to create an `Mvec` from a vector of u32.
    pub fn new(values: Vec<u32>) -> Result<Self, MvecError> {
        let mut non_zero_values = Vec::with_capacity(values.len());

        for value in values {
            if let Some(non_zero) = NonZeroU32::new(value) {
                non_zero_values.push(non_zero);
            } else {
                return Err(MvecError { kind: MvecErrorKind::ZeroValue });
            }
        }

        Ok(Mvec { values: non_zero_values })
    }
}

/// Provides a default `Mvec` value.
impl Default for Mvec {
    fn default() -> Self {
        Mvec::new(vec![2, 3, 5]).unwrap_or_else(|_| panic!("Default values for Mvec should be valid!"))
    }
}

impl From<ParseIntError> for MvecError {
    fn from(error: ParseIntError) -> Self {
        MvecError {
            kind: MvecErrorKind::ParseError(error),
        }
    }
}

// Implementing the Display trait for Mvec
impl fmt::Display for Mvec {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, value) in self.values.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", value)?;
        }
        write!(f, "]")
    }
}

impl fmt::Display for MvecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            MvecErrorKind::ZeroValue => write!(f, "zero value in mvec is not allowed"),
            MvecErrorKind::ParseError(e) => e.fmt(f),
        }
    }
}

impl FromStr for Mvec {
    type Err = MvecError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim_matches(|p| p == '[' || p == ']').split(',').collect();
        let mut values = Vec::with_capacity(parts.len());

        for part in parts {
            let value: u32 = part.trim().parse().map_err(MvecError::from)?;
            values.push(value);
        }

        Mvec::new(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_mvec_value() {
        let mvec = Mvec::new(vec![2, 3, 5]);
        assert!(matches!(mvec, Ok(_)));
    }

    #[test]
    fn test_invalid_mvec_value() {
        let mvec = Mvec::new(vec![0, 3, 5]);
        assert!(matches!(mvec, Err(_)));
    }

    #[test]
    fn test_string_parsing() {
        let mvec = "[2, 3, 5]".parse::<Mvec>();
        assert!(matches!(mvec, Ok(_)));
    }
}
