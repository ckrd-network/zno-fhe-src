use std::convert::TryFrom;
use std::convert::TryInto;

use crate::prelude::*;
use super::*;

#[derive(Debug, Clone)]
pub enum ConversionError {
    NegativeValue,
    NoValue,                   // The Context doesn't contain a value.
    ZeroValue,                 // The value is 0, which isn't allowed in NonZeroU32.
    OutOfRange(std::num::TryFromIntError), // The value is out of the range that can be represented by a u32.
}

impl core::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConversionError::NegativeValue => write!(f, "Negative value is not allowed in NonZeroU32."),
            ConversionError::NoValue => write!(f, "No value present in Context."),
            ConversionError::ZeroValue => write!(f, "Zero value is not allowed in NonZeroU32."),
            ConversionError::OutOfRange(e) => write!(f, "Value out of range for u32: {}", e),
        }
    }
}

impl std::error::Error for ConversionError {}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstructionError {
    kind: ConstructionErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructionErrorKind {
    NullPointer,
    InvalidParameter,
    Generic(String),
}

impl ConstructionError {
    /// Creates a new `ConstructionError` with the specified kind.
    pub fn new(kind: ConstructionErrorKind) -> Self {
        ConstructionError { kind }
    }
}

impl std::error::Error for ConstructionError {}

// Implement From for each error type to convert into ConstructionError
impl From<std::io::Error> for ConstructionError {
    fn from(e: std::io::Error) -> ConstructionError {
        ConstructionError {
            kind: ConstructionErrorKind::Generic(e.to_string()),
        }
    }
}

impl From<NullPointerError> for ConstructionError {
    fn from(_: NullPointerError) -> Self {
        ConstructionError {
            kind: ConstructionErrorKind::NullPointer,
        }
    }
}

impl core::fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            ConstructionErrorKind::InvalidParameter => write!(f, "invalid parameter"),
            ConstructionErrorKind::NullPointer => write!(f, "Received null pointer when constructing context"),
            ConstructionErrorKind::Generic(g) => g.fmt(f),
        }
    }
}

pub trait FheContext {
    type P: Parameters;
    type E: FheError;

    fn new(params: Self::P) -> Result<Self, Self::E>
    where
        Self: Sized;
}

/// Define the Rust struct to represent the C++ Context class
pub struct Context<C: FheContext> {
    inner: C,
}

// Define methods for the Rust struct Context.
// Logic common across implementations belongs here.
impl<C: FheContext> Context<C> {

    pub fn new<P: Parameters, E: FheError>(params: P) -> Result<Self, E> {
        let inner = C::new(params)?;
        Ok(Self { inner })
    }

    pub fn convert_to_vec(s: &str) -> Vec<i64> {
        s.split(',')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect()
    }

}

// Implement Display for printing, debugging, etc.
impl core::fmt::Display for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Context") // How this type name should appear
    }
}

// impl Getters for Context {
//     /// Get the `m` parameter from the `Context`.
//     ///
//     /// # Errors
//     ///
//     /// Returns an `MError` if `m` is zero or negative or if `m` is larger than `u32::MAX`.
//     // fn get_m(&self) -> Result<M, MError> {
//     //     // Call the C++ function through the FFI
//     //     let m_value = self.inner.getM();

//     //     // Convert the C++ result to Rust M enum
//     //     M::try_from(m_value)
//     // }

//     // Similarly, implement other getters...
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_with_valid_builder() {
        let builder = Builder::new();
        let context = builder.build();
        assert!(context.is_ok());
    }

    #[cfg(feature = "helib")]
    #[test]
    fn test_context_new_helib_bgv() {
        let params = Bgv::default(); // Replace with actual parameters
        let context: Context<Bgv> = Context::new(params).unwrap();
        // Use context
    }

    #[cfg(feature = "seal")]
    #[test]
    fn test_context_new_seal_bgv() {
        let params = Bgv::default(); ; // Replace with actual parameters
        let context: Context<Bgv> = Context::new(params).unwrap();
        // Use context
    }

    // #[ignore = "Incomplete SEAL FFI"]
    // #[test]
    // fn test_bgv_context_new() {
    //     // Set up the input parameters
    //     let context = Context::new(Parameters::default()).expect("BGV context creation should succeed");
    //     let actual_m = context.get_m().expect("Retrieving M value failed"); // Panic if get_m() returns an Err
    //     let expected_m = M::default();
    //     assert_eq!(actual_m, expected_m, "BGV scheme parameter M, should be set correctly");
    // }

    // #[ignore = "Incomplete SEAL FFI"]
    // #[test]
    // fn test_get_m_valid_value() {
    //     let context_result = Context::new(Parameters::default()); // Create your context
    //     let m = context_result
    //                 .expect("Expected to successfully retrieve Context")
    //                 .get_m()
    //                 .expect("Expected to successfully retrieve M");
    //     assert_eq!(m, M::default());
    // }

}
