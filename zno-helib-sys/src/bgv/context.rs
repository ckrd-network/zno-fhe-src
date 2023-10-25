use std::convert::TryFrom;
use std::convert::TryInto;

use crate::prelude::*;

// Bring `Context` to `bgv` module level, avoiding the need to include `ffi` in the path
#[cfg(feature = "helib")]
pub use crate::helib::bgv::*;
#[cfg(feature = "openfhe")]
pub use crate::openfhe::bgv::*;
#[cfg(feature = "seal")]
pub use crate::seal::bgv::*;

pub struct OptionalLong {
    pub value: Option<core::num::NonZeroU32>,
}

#[derive(Debug, Clone)]
pub enum ConversionError {
    NegativeValue,
    NoValue,                   // The ffi::OptionalLong doesn't contain a value.
    ZeroValue,                 // The value is 0, which isn't allowed in NonZeroU32.
    OutOfRange(std::num::TryFromIntError), // The value is out of the range that can be represented by a u32.
}

impl core::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConversionError::NegativeValue => write!(f, "Negative value is not allowed in NonZeroU32."),
            ConversionError::NoValue => write!(f, "No value present in ffi::OptionalLong."),
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

/// Define the Rust struct to represent the C++ Context class
pub struct Context {
    // This holds a pointer to the C++ object.
    // UniquePtr ensures proper destruction, preventing memory leaks.
    pub(crate) inner: cxx::UniquePtr<crate::helib::bgv::ffi::Context>,
}

// Define methods for the Rust struct helib::Context.
// Logic specific to the HElib implementation belongs here.
impl Context {

    // Create a new instance of the C++ object Context.
    // This is safe because we're not exposing the inner pointer directly.
    // Logic specific to the HElib implementation belongs here.
    pub fn new(params: crate::bgv::BGVParams) -> Result<Self, BGVError> {
        let mut params = params; // Make params mutable
        let cb = BGVContextBuilder::new()
                     .set_m(params.m)?;

        // Build BGV context. Consume the instance of BGVContextBuilder.
        // return a UniquePtr<ffi::Context>
        let cntxt = cb.build();
        match cntxt {
            Ok(context) => {
                // If successful, wrap the inner ffi::Context in a UniquePtr and return.
                let inner = context.inner;  // assuming context is of type ffi::Context
                Ok(Self { inner })
            },
            Err(_) => {
                // If there's an error during construction, return a corresponding BGVError.
                let e = ConstructionError::new(ConstructionErrorKind::Generic("An error occurred while building the BGV Context".into()));
                Err(BGVError::ConstructionError(e))
            }
        }
    }

    pub fn convert_to_vec(s: &str) -> Vec<i64> {
        s.split(',')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect()
    }


    /// Get the `m` parameter from the HElib `Context`.
    ///
    /// # Errors
    ///
    /// Returns an `Err` if `m` is zero or negative or if `m` is larger than `u32::MAX`.
    pub fn get_m(&self) -> Result<M, MError> {
        // Call the C++ function through the FFI
        let m_value = self.inner.getM();

        // Convert the C++ result to Rust M enum
        M::from_i64(m_value)
    }
}

// Implement Display for printing, debugging, etc.
impl core::fmt::Display for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Context") // How this type name should appear
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_with_valid_builder() {
        let builder = BGVContextBuilder::new();
        let context = builder.build();
        assert!(context.is_ok());
    }

    #[test]
    fn test_bgv_context_new() {
        // Set up the input parameters
        let context = Context::new(BGVParams::default()).expect("BGV context creation should succeed");
        let actual_m = context.get_m().expect("Retrieving M value failed"); // Panic if get_m() returns an Err
        let expected_m = M::new(4095).unwrap();
        assert_eq!(actual_m, expected_m, "BGV scheme parameter M, should be set correctly");
    }

    #[test]
    fn test_get_m_valid_value() {
        let context_result = Context::new(BGVParams::default()); // Create your context
        let m = context_result
                    .expect("Expected to successfully retrieve Context")
                    .get_m()
                    .expect("Expected to successfully retrieve M");
        assert_eq!(m, M::new(4095).unwrap());
    }

    // #[test]
    // fn test_set_m_zero() {
    //     let m = M::new(0);
    //     params = BGVParams { m.unwrap(), ..BGVParams::default() };
    //     let context_result = Context::new(params); // Create your context
    //     assert!(context_result.is_err(), "Expected error for m_value of 0, but got Ok");
    //     if let Err(err) = context_result {
    //         assert_eq!(err, BGVError::MError(MError { kind: MErrorKind::Zero }));
    //     }
    // }

    //     #[test]
    // fn test_get_m_negative() {
    //     // Mock/stub the FFI function to return -32
    //     // ...

    //     let context_result = setup_bgv_context_with_m(-32);

    //     let result = context_result
    //                 .expect("Expected to successfully retrieve Context")
    //                 .get_m();
    //     assert!(result.is_err());
    //     assert_eq!(result.unwrap_err().kind, MErrorKind::OutOfRange("Value must be in the range of 1 to u32::MAX".into()));
    // }

//     #[test]
//     fn test_get_m_overflow() {
//         // Mock/stub the FFI function to return a large value beyond u32::MAX
//         let context = setup_bgv_context_with_m(4095); // Create your context
//         assert!(matches!(context.get_m(), Err(MError { kind: MErrorKind::OutOfRange("Value must be in the range of 1 to u32::MAX".into()) })));
//     }

}
