use std::fmt;
use std::str::FromStr;

/// Represents the `bootstrappable` parameter in BGV.
///
/// In the BGV encryption scheme as implemented by HElib, the `bootstrappable` parameter indicates
/// whether the created context supports bootstrapping. Bootstrapping is a fundamental operation
/// in homomorphic encryption schemes that helps in reducing the noise of ciphertexts, allowing
/// more operations to be performed on encrypted data without the need for decryption.
///
/// Setting this flag ensures that the created context is equipped to handle bootstrapping. This
/// might involve setting additional parameters and structures in the underlying implementation.
///
/// # Errors
///
///   - No specific errors. This type just represents a boolean flag indicating if bootstrapping
///     is enabled or not.
///
/// # Panic
///
///  Can panic under certain conditions. For instance, using `unwrap()` on a `Result` that contains an
/// error will cause a panic. It's recommended to handle errors gracefully using pattern matching or methods
/// like `is_ok()` and `is_err()` before unwrapping.
///
/// # Example
///
/// ```
/// # use your_crate_name::Bootstrappable;  // Replace `your_crate_name` with the name of your crate
/// let boot_flag = Bootstrappable::new(true).expect("Failed to create Bootstrappable");
/// assert_eq!(boot_flag.to_string(), "true");
/// ```
///
#[derive(Debug, PartialEq)]
pub enum Bootstrappable {
    Some(bool),
    Err(String),  // Simple error variant for any potential parsing errors.
}

impl Bootstrappable {
    /// Create a `Bootstrappable` variant from a given boolean.
    pub fn new(value: bool) -> Self {
        Bootstrappable::Some(value)
    }
}

impl fmt::Display for Bootstrappable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bootstrappable::Some(value) => write!(f, "{}", value),
            Bootstrappable::Err(e) => write!(f, "Error: {}", e),
        }
    }
}

impl FromStr for Bootstrappable {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "true" => Ok(Bootstrappable::Some(true)),
            "false" => Ok(Bootstrappable::Some(false)),
            _ => Ok(Bootstrappable::Err(format!("Invalid value for Bootstrappable: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bootstrappable_value() {
        let boot_flag = Bootstrappable::new(true);
        assert_eq!(boot_flag, Bootstrappable::Some(true));
    }

    #[test]
    fn test_invalid_string_bootstrappable_value() {
        let boot_flag = "maybe".parse::<Bootstrappable>();
        assert!(matches!(boot_flag, Ok(Bootstrappable::Err(_))));
    }

    #[test]
    fn test_valid_string_bootstrappable_value() {
        let boot_flag = "true".parse::<Bootstrappable>();
        assert!(matches!(boot_flag, Ok(Bootstrappable::Some(true))));
    }
}
