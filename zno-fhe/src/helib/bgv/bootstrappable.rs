use std::fmt;
use core::str::FromStr;

use crate::prelude::*;

/// Represents the bootstrapping capability status in the BGV scheme.
///
/// In the BGV encryption scheme as implemented by HElib, the `bootstrappable` parameter indicates
/// whether the created context has been configured for bootstrapping.
/// Bootstrapping is a crucial operation in homomorphic encryption, allowing the noise
/// reduction in ciphertexts and enabling more computations on encrypted data without decryption.
///
/// This enum reflects the state of bootstrapping capability:
/// - `Enabled`: The context is configured to support bootstrapping. Additional parameters and structures in the underlying implementation have been set.
/// - `Disabled`: The context does not support bootstrapping.
/// - `None`: No explicit setting provided, the context has not been configured.  Generally this means the logic associated with the `Bootstrap` parameter has not been invoked.
///
/// # Usage in this FFI Implementation:
///
/// This FFI implementation uses a boolean to indicate the `bootstrappable` state.
/// Where `true` indicates that the scheme is setup to be bootstrappable, and `false` indicates that it is not.
///
/// # Usage in HElib:
///
/// Bootstrapping in HElib is an advanced feature that requires careful configuration.
/// Users should refer to HElib's official documentation or relevant publications for detailed guidelines on using bootstrapping.
///
/// # Errors
///
/// Errors can occur when trying to create a `Bootstrappable` from a string. The operation
/// might fail if the input string does not correctly describe a bootstrapping state.
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
/// # use crate::Bootstrappable;
/// let bootstrappable = Bootstrappable::new("enabled").expect("Invalid state");
/// assert_eq!(bootstrappable, Bootstrappable::Enabled);
/// ```
///
#[derive(Debug, PartialEq)]
pub enum Bootstrappable {
    None,
    Enabled,
    Disabled,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrappableError {
    pub kind: BootstrappableErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BootstrappableErrorKind {
    InvalidString(String),
    // other kinds of errors can be added here
}

impl core::fmt::Display for BootstrappableError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match &self.kind {
            BootstrappableErrorKind::InvalidString(s) =>
                write!(f, "Invalid string for Bootstrappable: {}", s),
            // handle other errors here
        }
    }
}

impl std::error::Error for BootstrappableError {}

impl Bootstrappable {
    /// Creates a new `Bootstrappable` instance from a string representation.
    /// The string can be "none", "enabled", or "disabled" (case-insensitive).
    pub fn new(s: &str) -> Result<Self, BootstrappableError> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Bootstrappable::None),
            "enabled" => Ok(Bootstrappable::Enabled),
            "disabled" => Ok(Bootstrappable::Disabled),
            _ => Err(BootstrappableError {
                kind: BootstrappableErrorKind::InvalidString(s.to_string())
            }),
        }
    }
}

impl Default for Bootstrappable {
    fn default() -> Self {
        Bootstrappable::None
    }
}

impl core::str::FromStr for Bootstrappable {
    type Err = BootstrappableError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Bootstrappable::new(s)
    }
}

impl core::fmt::Display for Bootstrappable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", match self {
            Bootstrappable::None => "none",
            Bootstrappable::Enabled => "enabled",
            Bootstrappable::Disabled => "disabled",
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bootstrappable_value() {
        let bootstrappable = Bootstrappable::new("enabled").unwrap();
        assert_eq!(bootstrappable.to_string(), "enabled");
    }

    #[test]
    fn test_invalid_string_bootstrappable_value() {
        let bootstrappable = "not valid".parse::<Bootstrappable>();
        assert!(bootstrappable.is_err());
    }

    #[test]
    fn test_valid_string_bootstrappable_value() {
        let bootstrappable = "disabled".parse::<Bootstrappable>().unwrap();
        assert_eq!(bootstrappable, Bootstrappable::Disabled);
    }

    #[test]
    fn test_case_insensitivity() {
        let bootstrappable = "DiSaBlEd".parse::<Bootstrappable>().unwrap();
        assert_eq!(bootstrappable, Bootstrappable::Disabled);
    }
}
