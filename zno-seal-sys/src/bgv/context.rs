use std::convert::TryFrom;
use std::convert::TryInto;

use crate::prelude::*;
use super::*;

/// Define the Rust struct to represent the C++ Context class
pub struct Context {
    // This holds a pointer to the C++ object.
    // UniquePtr ensures proper destruction, preventing memory leaks.
    pub(crate) inner: cxx::UniquePtr<crate::seal::bgv::ffi::Context>,
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

    // #[ignore = "Incomplete HELib FFI"]
    // #[test]
    // fn test_bgv_context_new() {
    //     // Set up the input parameters
    //     let context = Context::new(Parameters::default()).expect("BGV context creation should succeed");
    //     let actual_m = context.get_m().expect("Retrieving M value failed"); // Panic if get_m() returns an Err
    //     let expected_m = M::default();
    //     assert_eq!(actual_m, expected_m, "BGV scheme parameter M, should be set correctly");
    // }

    // #[ignore = "Incomplete HELib FFI"]
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
