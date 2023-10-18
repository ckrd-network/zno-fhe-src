use std::default::Default;

/// Represents the bootstrapping mode in the BGV scheme as implemented in HElib.
///
/// BGV, originally a leveled homomorphic encryption scheme, allows computations up to a certain level (or depth) before
/// noise overshadows the ciphertext. Bootstrapping rejuvenates ciphertexts by decreasing noise, thereby
/// resetting computational depth/level and enabling further operations. This advancement shifts BGV from being
/// merely "leveled" to "fully" homomorphic, theoretically allowing unlimited computations, provided
/// bootstrapping is used judiciously to control noise.
///
/// The `Bootstrap` enum represents modes for this pivotal process:
/// - `None`: No bootstrapping. Limits computations to the initial 'levels' set by the encryption parameters.
/// - `Thick`: Intensive bootstrapping with superior noise reduction, at the expense of higher computational cost.
/// - `Thin`: Less intensive, faster bootstrapping, but with less noise reduction.
/// - `Error`: Represents a parsing error, used when an input string doesn't match any of the above modes.
///
/// Although bootstrapping enables limitless computations, its practicality hinges on the efficiency of the
/// bootstrapping process (dictated by cryptographic parameters and available resources) and the balance
/// between computational cost and accuracy in applications.
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
/// # use your_crate_name::Bootstrap;  // Replace `your_crate_name` with the actual name of your crate
/// let mode = Bootstrap::Thick;
/// assert_eq!(mode, Bootstrap::Thick);
/// ```
///
#[derive(Debug, PartialEq)]
pub enum Bootstrap {
    None,
    Thick,
    Thin,
    Error(String), // Error variant to handle parsing errors.
}

impl fmt::Display for Bootstrap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Bootstrap::None => write!(f, "None"),
            Bootstrap::Thick => write!(f, "Thick"),
            Bootstrap::Thin => write!(f, "Thin"),
            Bootstrap::Error(err) => write!(f, "Error: {}", err),
        }
    }
}

impl FromStr for Bootstrap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "none" => Ok(Bootstrap::None),
            "thick" => Ok(Bootstrap::Thick),
            "thin" => Ok(Bootstrap::Thin),
            _ => Ok(Bootstrap::Error(format!("Invalid value for Bootstrap: {}", s))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Bootstrap::None.to_string(), "None");
        assert_eq!(Bootstrap::Thick.to_string(), "Thick");
        assert_eq!(Bootstrap::Thin.to_string(), "Thin");
        assert_eq!(Bootstrap::Error("Invalid".to_string()).to_string(), "Error: Invalid");
    }

    #[test]
    fn test_from_str() {
        assert_eq!("None".parse(), Ok(Bootstrap::None));
        assert_eq!("Thick".parse(), Ok(Bootstrap::Thick));
        assert_eq!("Thin".parse(), Ok(Bootstrap::Thin));
        assert_eq!("Invalid".parse::<Bootstrap>(), Ok(Bootstrap::Error("Invalid value for Bootstrap: Invalid".to_string())));
    }
}
