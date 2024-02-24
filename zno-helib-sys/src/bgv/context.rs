use std::convert::TryFrom;
use std::convert::TryInto;

use crate::prelude::*;
use super::*;

// Bring `Context` to `bgv` module level, avoiding the need to include `ffi` in the path
#[cfg(feature = "helib")]
pub use crate::helib::bgv::*;
#[cfg(feature = "openfhe")]
pub use crate::openfhe::bgv::*;
#[cfg(feature = "seal")]
pub use crate::seal::bgv::*;

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
    pub fn new(params: crate::bgv::Parameters) -> Result<Self, BGVError> {
        let mut params = params; // Make params mutable
        let cb: Builder = Builder::new()
                     .set(params.m.into())?
                     .set(params.p.into())?
                     .set(params.r.into())?
                    // optional or conditional: You can call build without these.
                    // https://users.rust-lang.org/t/builder-pattern-in-rust-self-vs-mut-self-and-method-vs-associated-function/72892/2
                    // https://dev.to/mindflavor/rust-builder-pattern-with-types-3chf
                     .set(params.bits.into())?
                     .set(params.c.into())?
                    //  .set(params.l.into())
                    //  .set(params.scale.into())
                     .set(params.gens.into())?
                     .set(params.ords.into())?
                     .set(params.mvec.into())?
                    // Quote:
                    //     buildModChain must be called BEFORE the context is made
                    //     botstrappable (else the "powerful" basis is not initialized correctly.
                    // .set(params.modulus_chain.into())
                     .set(params.bootstrap.into())?
                     ;

        // Build BGV context. Consume the instance of Builder.
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

}

// Implement Display for printing, debugging, etc.
impl core::fmt::Display for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Context") // How this type name should appear
    }
}

impl Getters for Context {
    /// Get the `m` parameter from the `Context`.
    ///
    /// # Errors
    ///
    /// Returns an `MError` if `m` is zero or negative or if `m` is larger than `u32::MAX`.
    fn get_m(&self) -> Result<M, MError> {
        // Call the C++ function through the FFI
        let m_value = self.inner.getM();

        // Convert the C++ result to Rust M enum
        M::try_from(m_value)
    }

    // Similarly, implement other getters...
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_with_valid_builder() {
        let builder = Builder::new();
        let context = builder.build();
        assert!(context.is_ok());
    }

    #[ignore = "Incomplete HELib FFI"]
    #[test]
    fn test_bgv_context_new() {
        // Set up the input parameters
        let context = Context::new(Parameters::default()).expect("BGV context creation should succeed");
        let actual_m = context.get_m().expect("Retrieving M value failed"); // Panic if get_m() returns an Err
        let expected_m = M::default();
        assert_eq!(actual_m, expected_m, "BGV scheme parameter M, should be set correctly");
    }

    #[ignore = "Incomplete HELib FFI"]
    #[test]
    fn test_get_m_valid_value() {
        let context_result = Context::new(Parameters::default()); // Create your context
        let m = context_result
                    .expect("Expected to successfully retrieve Context")
                    .get_m()
                    .expect("Expected to successfully retrieve M");
        assert_eq!(m, M::default());
    }

}
