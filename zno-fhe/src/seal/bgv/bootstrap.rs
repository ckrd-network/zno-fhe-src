use std::default::Default;
use std::fmt;
use std::str::FromStr;

use crate::prelude::*;

/// Represents the bootstrapping mode in the BGV scheme as implemented in SEAL.
///
/// BGV, a leveled homomorphic encryption scheme, allows computations up to a certain level (or depth) before
/// noise overshadows the ciphertext. Bootstrapping rejuvenates ciphertexts by decreasing noise, this
/// resets the computational depth/level and enables further operations. This advancement shifts BGV from being
/// merely "leveled" to "fully" homomorphic, theoretically allowing unlimited computations, provided
/// bootstrapping is used judiciously to control noise.
///
/// The `Bootstrap` enum represents modes for this process:
/// - `None`: No bootstrapping. Limits computations to the initial 'levels' set by the encryption parameters.
/// - `Thick`: Intensive bootstrapping with superior noise reduction, at the expense of higher computational cost.
/// - `Thin`: Less intensive, faster bootstrapping, but with less noise reduction.
/// - `Error`: Represents a parsing error, used when an input string doesn't match any of the above modes.
///
/// Although bootstrapping enables limitless computations, its practicality hinges on the efficiency of the
/// bootstrapping process (dictated by cryptographic parameters and available resources) and the balance
/// between computational cost and accuracy in applications.
///
/// Users should refer to SEAL's official documentation or relevant publications for detailed guidelines on configuring the `bootstrap` mode.
///
/// # Errors
///
/// This enum's methods do not return `Err` values in a `Result`, as parsing simply yields an `Error` variant
/// when it encounters an unexpected input. This design decision encapsulates "errors" within the enum itself,
/// allowing for straightforward handling of unexpected states without complicating function signatures.
///
/// # Panic
///
/// The methods of this enum do not panic, as they're designed to handle errors internally by using the `Error`
/// variant. This approach ensures that all possible states are accounted for and that the calling code can
/// handle these states gracefully without the risk of unexpected panics.
///
/// # Example
///
/// ```
/// # use crate::Bootstrap;
/// let bootstrap = Bootstrap::from_str("thin").expect("Failed to create Bootstrap");
/// assert_eq!(bootstrap.to_string(), "thin");
/// assert!(matches!(bootstrap, Ok(Bootstrap::Thin)));
/// ```
///
#[derive(Debug, PartialEq)]
pub enum Bootstrap {
    None,
    Thin,
    Thick,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BootstrapError;

impl core::fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Invalid value for bootstrap. Valid values are: 'none', 'thick', 'thin'.")
    }
}

impl std::error::Error for BootstrapError {}

impl Bootstrap {
    /// Creates a new `Bootstrap` from a string slice.
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice that holds the name of the variant
    ///
    /// # Returns
    ///
    /// * `Result` - `Ok` if the string corresponds to a known variant,
    ///   `Err` otherwise.
    pub fn new(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Bootstrap::None),
            "thin" => Ok(Bootstrap::Thin),
            "thick" => Ok(Bootstrap::Thick),
            _ => Err(format!("'{}' is not a valid variant of Bootstrap", s)),
        }
    }
}

impl FromStr for Bootstrap {
    type Err = BootstrapError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Bootstrap::None),
            "thin" => Ok(Bootstrap::Thin),
            "thick" => Ok(Bootstrap::Thick),
            _ => Err(BootstrapError),
        }
    }
}

/// Converts from `Bootstrap` to `Metric`.
///
/// This implementation allows a `Bootstrap` to be converted into a `Metric`
/// using the `into` method, which is part of the `Into` trait.
///
/// # Examples
///
/// ```
/// let bootstrap = Bootstrap::new();
/// let metric: Metric = bootstrap.into();
/// ```
impl Into<Metric> for Bootstrap {
    fn into(self) -> Metric {
        Metric::Bootstrap(self)
    }
}

// Implementing the Default trait for Bootstrap
impl Default for Bootstrap {
    /// Provides a default value for the Bootstrap type, which is `Bootstrap::Thick`.
    fn default() -> Self {
        Bootstrap::Thick
    }
}

// Implementing the Display trait for Bootstrap
impl core::fmt::Display for Bootstrap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Bootstrap::None => write!(f, "none"),
            Bootstrap::Thin => write!(f, "thin"),
            Bootstrap::Thick => write!(f, "thick"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_bootstrap_values() {
        let none = Bootstrap::from_str("none");
        let thin = Bootstrap::from_str("thin");
        let thick = Bootstrap::from_str("thick");

        assert!(matches!(none, Ok(Bootstrap::None)));
        assert!(matches!(thin, Ok(Bootstrap::Thin)));
        assert!(matches!(thick, Ok(Bootstrap::Thick)));
    }

    #[test]
    fn test_invalid_bootstrap_value() {
        let bootstrap = Bootstrap::from_str("invalid");
        assert!(matches!(bootstrap, Err(_)));
    }

    #[test]
    fn test_default_bootstrap() {
        // It should create a Bootstrap::Thick by default.
        let default_bootstrap: Bootstrap = Default::default();
        assert_eq!(default_bootstrap, Bootstrap::Thick);
    }

        #[test]
    fn test_bootstrap_new_none() {
        let bootstrap = Bootstrap::new("none").unwrap();
        assert_eq!(bootstrap, Bootstrap::None);
    }

    #[test]
    fn test_bootstrap_new_thin() {
        let bootstrap = Bootstrap::new("thin").unwrap();
        assert_eq!(bootstrap, Bootstrap::Thin);
    }

    #[test]
    fn test_bootstrap_new_thick() {
        let bootstrap = Bootstrap::new("thick").unwrap();
        assert_eq!(bootstrap, Bootstrap::Thick);
    }

    #[test]
    fn test_bootstrap_new_case_insensitive() {
        let bootstrap = Bootstrap::new("ThIn").unwrap();
        assert_eq!(bootstrap, Bootstrap::Thin);
    }

    #[test]
    fn test_bootstrap_new_invalid() {
        assert!(Bootstrap::new("invalid").is_err());
    }
}
