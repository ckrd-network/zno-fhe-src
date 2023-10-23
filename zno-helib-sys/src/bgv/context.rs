use crate::prelude::*;
// Bring `Context` to `bgv` module level, avoiding the need to include `ffi` in the path
#[cfg(feature = "helib")]
pub use crate::helib::bgv::*;
#[cfg(feature = "openfhe")]
pub use crate::openfhe::bgv::*;
#[cfg(feature = "seal")]
pub use crate::seal::bgv::*;

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
    pub inner: cxx::UniquePtr<crate::helib::bgv::ffi::Context>,
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
        match cb.build() {
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
        let m_long = self.inner.getM(); // Call FFI method

        // Check for overflow and zero value
        if m_long > 0 && m_long <= (u32::MAX as i64) {
            M::new(m_long as u32)
        } else if m_long == 0 {
            Err(MError { kind: MErrorKind::Zero })
        } else {
            Err(MError { kind: MErrorKind::OutOfRange("Value out of range of u32".to_string()) })
        }
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

    // Helper function to create a Context with default or dummy data.
    fn setup_bgv_context_with_m(m_value: u32) -> Context {
        let m = M::new(m_value).expect("Should be able to create a new M");

        let params = BGVParams {
            m,
            ..Default::default() // ... other fields with dummy or default data
        };

        Context::new(params).expect("BGV context creation")
    }

    #[test]
    fn test_build_with_valid_builder() {
        let builder = BGVContextBuilder::new();
        let context = builder.build();
        assert!(context.is_ok());
    }

    #[test]
    fn test_bgvcontext_new() {
        // Set up the input parameters
        let context = setup_bgv_context_with_m(32);

        let actual_m = context.get_m().unwrap(); // this will panic if get_m() returns an Er

        let expected_m = M::new(4095).unwrap();
        assert_eq!(actual_m, expected_m, "BGV scheme parameter M, should be set correctly");
    }

    // #[test]
    // fn test_get_m_valid() {
    //     // Set up your Context and BGV with valid parameters.
    //     // This part depends on your specific setup for creating a Context.

    //     let context = setup_bgv_context_with_m(4095); // Create your context
    //     let expected_m = context.get_m().unwrap();
    //     assert_eq!(m, M::Some(core::num::NonZeroU32::new(4095).unwrap())); // Use expected value
    // }

    #[test]
    fn test_get_m_zero() {
        // Set up your Context and BGV in such a way that getM would return 0.
        // This might be tricky, because it depends on the internal state of the Context in C++.

        let context = setup_bgv_context_with_m(0); // Create your context
        assert_eq!(context.get_m(), Err(MError { kind: MErrorKind::Zero }));
    }

    // #[test]
    // fn test_get_m_overflow() {
    //     // Set up your Context and BGV in such a way that getM would return a value larger than u32::MAX.
    //     // This scenario might be unrealistic depending on the constraints of your actual C++ library.

    //     let context = setup_bgv_context_with_m(4095); // Create your context
    //     assert!(matches!(context.get_m(), Err(MError { kind: MErrorKind::ParseError(_) })));
    // }
}
