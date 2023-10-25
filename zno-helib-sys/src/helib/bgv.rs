use core::pin::Pin;
use core::fmt;
use cxx::CxxString;
use cxx::CxxVector;
use cxx::UniquePtr;
// use std::convert::TryInto;
use std::ptr;
use std::{fmt::{Display, Formatter}, error::Error};

// Import the BGV struct and its fields
use crate::bgv::*;

#[cxx::bridge(namespace="helib")]
pub mod ffi {
    unsafe extern "C++" {
        include!("zno-helib-sys/ffi/ffi_wrapper.h");

        // Non-template representation for BGV.
        type Context;
        type BGVContextBuilder;

        fn to_std_vector(rust_vec: &Vec<i64>) -> UniquePtr<CxxVector<i64>>;

        fn new_bgv_builder() -> UniquePtr<BGVContextBuilder>;
        fn set_m(builder: UniquePtr<BGVContextBuilder>, m: u32) -> UniquePtr<BGVContextBuilder>;

        fn build_ptr(builder: UniquePtr<BGVContextBuilder>) -> UniquePtr<Context>;
        // fn is_bootstrappable(builder: Pin<&mut BGVContextBuilder>, flag: bool);
        // fn set_bits(builder: Pin<&mut BGVContextBuilder>, bits: i64);
        // fn set_c(builder: Pin<&mut BGVContextBuilder>, c: i64);
        // fn set_gens(builder: Pin<&mut BGVContextBuilder>, gens: &CxxVector<i64>);
        // fn set_m(builder: Pin<&mut BGVContextBuilder>, m: i64);
        // fn set_mvec(builder: Pin<&mut BGVContextBuilder>, mvec: &CxxVector<i64>);
        // fn set_ords(builder: Pin<&mut BGVContextBuilder>, ords: &CxxVector<i64>);
        // fn set_p(builder: Pin<&mut BGVContextBuilder>, p: i64);
        // fn set_r(builder: Pin<&mut BGVContextBuilder>, r: i64);
        // fn set_thickboot(builder: Pin<&mut BGVContextBuilder>);
        // fn set_thinboot(builder: Pin<&mut BGVContextBuilder>);

        fn getM(self: &Context) -> i64;
    }

    extern "Rust" {
        type M;
        type MError;
    }
}

/// - Rust-side Null Pointer Check: On receipt of a raw pointer from C++,
///   immediately check if it's null before converting it to a safe Rust type.
///   If it's null, return an error.
/// - C++-side Error Handling: C++ functions called from Rust should signal
///   when an error has occurred. This could be returning a null pointer,
///   or some other error signaling mechanism.
/// - Error Type: The `NullPointerError` type represents an error in the
///   case of a null pointer.
///
/// A more general FFIError type represents other kinds of errors that can occur in the FFI context.

// `NullPointerError`: An error type for null pointer exceptions
#[derive(Debug, Clone)]
pub struct NullPointerError;

impl std::error::Error for NullPointerError {}

impl Display for NullPointerError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "Received null pointer from C++ library")
    }
}

#[derive(Debug, Clone)]
pub enum FFIError {
    NullPointer(NullPointerError),
    // Other FFI-related errors can be added here
    CppException(String), // This can represent an exception thrown by C++
    // ...
}

impl Error for FFIError {}

impl Display for FFIError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FFIError::NullPointer(err) => write!(f, "Null pointer error: {}", err),
            FFIError::CppException(err) => write!(f, "C++ exception: {}", err),
            // other cases as needed
        }
    }
}

// Logic specific to the HElib implementation belongs here.
pub struct Builder {
    // Holds a pointer to the C++ object
    pub inner: cxx::UniquePtr<ffi::BGVContextBuilder>,
}

impl Builder {
    // Constructs a new Builder
    pub fn new() -> Self {
        Builder {
            inner: ffi::new_bgv_builder(),
        }
    }

    // Note: This consumes the Builder instance when
    // `cb.build()` is called, so you won't be able to use it afterward.
    pub fn build(self) -> Result<Context, FFIError> {
        // This call is safe because it transfers ownership of the Context
        // to the UniquePtr, which ensures it will be cleaned up correctly.
        let ctx_ptr: UniquePtr<ffi::Context> = ffi::build_ptr(self.inner);
        // Check if the pointer is null
        if ctx_ptr.is_null() {
            return Err(FFIError::NullPointer(NullPointerError));
        }
        Ok(Context { inner: ctx_ptr })
    }

    // Similarly, implement other setters following the same pattern...

}

impl Setters for Builder {
    fn set_m<T, E>(mut self, value: T) -> Result<Self, MError>
    where
        Self: Sized,
        T: ToU32<E>,
        E: Into<MError>,
    {
        let value_u32 = value.to_u32().map_err(Into::into)?;
        self.inner = ffi::set_m(self.inner, value_u32);
        Ok(self)
    }

    // Similarly, implement other setters following the same pattern...
}

/// #Example
///
/// fn main() {
///     // Initialize the builder.
///     // Pin the builder before passing it to the function
///     let mut builder = ffi::make_context_builder();
///     let pinned_builder = Pin::new(&mut builder);
///
///
///     // Set the parameters you need for your context.
///     // These functions are just examples; replace them with the actual functions exposed by your FFI.
///     builder.set_m(16); // Set the cyclotomic order
///     builder.set_p(2); // Set the plaintext prime
///     builder.set_r(1); // Set the Hensel lifting
///     builder.set_bits(300); // Set the bits
///     builder.set_c(2); // Set the number of columns in key-switching matrix
///     builder.set_gens(Vec::new()); // Provide an empty vector, replace with your actual data as needed
///     builder.set_ords(Vec::new()); // Provide an empty vector, replace with your actual data as needed
///
///     // Decide on bootstrappability
///     builder.bootstrappable(true);
///     // If bootstrappable, you might want to set more parameters like `mvec`, `thickboot`, or `thinboot`.
///     // ...
///
///     // Build the context from the builder.
///     let context: UniquePtr<ffi::Context> = ffi::build_context(&mut builder);
///     let context: UniquePtr<ffi::Context> = ffi::build_ptr(pinned_builder);
///     // Now `context` is ready to be used for further computations.
///     // ...
///
///     // The `Context` will automatically be cleaned up when `context` goes out of scope.
/// }
///
/// Please replace the placeholder function calls with the actual API provided by the C++ library you are interfacing with, as the exact function names and parameters will depend on the HElib API.
///
/// Also, replace your_crate_name with the name of your actual crate. This example assumes you are using the cxx crate for FFI, and that you have a module named ffi which exposes the functions from C++.
///

#[cfg(test)]
mod tests {
    use super::*;
    use cxx::CxxVector;

    // Helper function to create a Context with default or dummy data.
    fn setup_bgv_context(parameters: Parameters) -> Context {
        Context::new(parameters).expect("BGV context creation")
    }

    #[test]
    fn test_context_with_defaults() {
        let context = Context::new(Parameters::default());
        assert!(context.is_ok());
    }

    // #[test]
    // fn test_bgv_context_builder_build_null_pointer() {
    //     // Mock/stub the ffi::build_ptr to return a null pointer
    //     // ...
    //     let builder = Builder::new();
    //     let result = Context::new(parameters).expect("BGV context creation");
    //     assert!(result.is_err());
    //     if let Err(err) = result {
    //         assert_eq!(err, FFIError::NullPointer(NullPointerError));
    //     } else {
    //         panic!("Expected an error!");
    //     }
    // }

    // #[test]
    // fn test_bgv_context_builder_build_cpp_exception() {
    //     // Mock/stub the ffi::build_ptr to simulate a C++ exception
    //     // ...
    //     let builder = Builder::new();
    //     let result = builder.build();
    //     assert!(result.is_err());
    //     if let Err(err) = result {
    //         match err {
    //             FFIError::CppException(msg) => assert!(msg.contains("Expected C++ exception message")),
    //             _ => panic!("Expected a C++ exception!"),
    //         }
    //     } else {
    //         panic!("Expected an error!");
    //     }
    // }

    // #[test]
    // fn test_bgvcontext_new() {
    //     // Set up the input parameters
    //     let context = setup_bgv_context_with_m(32);
    //     let actual_m = context.get_m().unwrap(); // this will panic if get_m() returns an Er
    //     let expected_m = M::new(4095).unwrap();
    //     assert_eq!(actual_m, expected_m, "BGV scheme parameter M, should be set correctly");
    // }

}
