use core::pin::Pin;
use core::fmt;
use cxx::CxxString;
use cxx::CxxVector;
use cxx::UniquePtr;
// use std::convert::TryInto;

// Import the BGV struct and its fields
use crate::bgv::*;

#[cxx::bridge(namespace="helib")]
pub mod ffi {

    unsafe extern "C++" {
        include!("zno-helib-sys/ffi/ffi_wrapper.h");


        // Non-template representation for BGV.
        type Context;

        fn getM(self: &Context) -> i64; // matching the C++ 'long' type

        type BGVContextBuilder;

        // Representations of the C++ ContextBuilder methods in Rust.
        // Assuming the builder methods don't need parameters from Rust.
        // fn build(self: &BGVContextBuilder) -> UniquePtr<bgv::ffi::Context>;

        // fn makeBGVContextBuilder() -> UniquePtr<BGVContextBuilder>;

        // fn from_std_vector(rustVec: &CxxVector<i64>) -> Vec<i64>;

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

    }

    // // External Rust functions, used to convert Rust types into types that can be used in C++:
    // extern "Rust" {
    //     fn convert_to_vec(s: &str) -> Vec<i64>;
    // }
}

// Logic specific to the HElib implementation belongs here.
pub struct BGVContextBuilder {
    // Holds a pointer to the C++ object
    inner: UniquePtr<ffi::BGVContextBuilder>,
}

impl BGVContextBuilder {
    // Constructs a new BGVContextBuilder
    pub fn new() -> Self {
        BGVContextBuilder {
            inner: ffi::new_bgv_builder(),
        }
    }

    // Note: This consumes the BGVContextBuilder instance when
    // `cb.build()` is called, so you won't be able to use it afterward.
    pub fn build(self) -> UniquePtr<ffi::Context> {
        // This call is safe because it transfers ownership of the Context
        // to the UniquePtr, which ensures it will be cleaned up correctly.
        ffi::build_ptr(self.inner)
    }

    // pub fn set_m<T: ToU32>(mut self, value: T) -> Result<Self, MError> {
    //     let value_u32 = value.to_u32()?; // Propagate the error if `to_u32` fails.
    //     self.inner = ffi::set_m(self.inner, value_u32); // Assume ffi::set_m can't fail for now.
    //     Ok(self)
    // }

    pub fn set_m<T, E>(mut self, value: T) -> Result<Self, MError>
    where
        T: ToU32<E>,
        E: Into<MError>, // Some way to convert E into MError
    {
        let value_u32 = value.to_u32().map_err(Into::into)?; // Converts E into MError
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
/// Rust's safety principles apply, and the UniquePtr<ffi::Context> ensures that when the Context goes out of scope, it will be properly cleaned up (its destructor will be called), preventing memory leaks.
///
/// Define the Rust struct to represent the C++ Context class
pub struct Context {
    // This holds a pointer to the C++ object.
    // UniquePtr ensures proper destruction, preventing memory leaks.
    inner: UniquePtr<ffi::Context>,
}

// Define methods for the Rust struct helib::Context.
// Logic specific to the HElib implementation belongs here.
impl Context {

    pub fn convert_to_vec(s: &str) -> Vec<i64> {
        s.split(',')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect()
    }

    // Create a new instance of the C++ object Context.
    // This is safe because we're not exposing the inner pointer directly.
    // Logic specific to the HElib implementation belongs here.
    pub fn new(params: crate::bgv::BGVParams) -> Result<Self, BGVError> {
        let mut params = params; // Make params mutable
        let cb = BGVContextBuilder::new()
                     .set_m(params.m)?;

        // Build BGV context. Consume the instance of BGVContextBuilder.
        // return a UniquePtr<ffi::Context>
        let inner = cb.build();
        Ok(Self { inner })
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
impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Context") // How this type name should appear
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a BGVContext with default or dummy data.
    fn setup_bgv_context_with_m(m_value: u32) -> BGVContext {
        let m = M::new(m_value).expect("Should be able to create a new M");

        let params = BGVParams {
            m,
            ..Default::default() // ... other fields with dummy or default data
        };

        BGVContext::new(&params)
    }

    #[test]
    fn test_bgvcontext_new() {
        // Set up the input parameters
        let context = setup_bgv_context_with_m(32);

        // Validate the results
        // This assumes the existence of a method `get_m` or similar that you can use to check the state of the context.
        // Replace `get_m` with an actual method or way to observe the state of `BGVContext`.
        assert_eq!(context.get_m(), Some(M::Some(NonZeroU32::new(32).unwrap())), "M value should be set correctly");
    }

        #[test]
    fn test_get_m_valid() {
        // Set up your Context and BGV with valid parameters.
        // This part depends on your specific setup for creating a Context.

        let context = setup_bgv_context_with_m(4095); // Create your context
        let m = context.get_m().unwrap();
        assert_eq!(m, M::Some(NonZeroU32::new(4095).unwrap())); // Use expected value
    }

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
