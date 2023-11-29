use core::pin::Pin;
use core::fmt;
use core::convert::TryFrom;
use core::convert::TryInto;
use std::ptr;
use std::{fmt::{Display, Formatter}, error::Error};

use cxx::CxxString;
use cxx::CxxVector;
use cxx::UniquePtr;

// Import the BGV struct and its fields
use crate::bgv::*;
use crate::prelude::*;

#[cxx::bridge(namespace="helib")]
pub mod ffi {
    unsafe extern "C++" {
        include!("zno-helib-sys/ffi/ffi_wrapper.h");

        // Non-template representation for BGV.
        type Context;
        type BGVContextBuilder;

        fn to_std_vector(rust_vec: &Vec<i64>) -> UniquePtr<CxxVector<i64>>;

        fn new_bgv_builder() -> UniquePtr<BGVContextBuilder>;

        fn set_bits(builder: UniquePtr<BGVContextBuilder>, bits: u32) -> UniquePtr<BGVContextBuilder>;
        fn set_c(builder: UniquePtr<BGVContextBuilder>, c: u32) -> UniquePtr<BGVContextBuilder>;
        fn set_m(builder: UniquePtr<BGVContextBuilder>, m: u32) -> UniquePtr<BGVContextBuilder>;
        fn set_p(builder: UniquePtr<BGVContextBuilder>, p: u32) -> UniquePtr<BGVContextBuilder>;
        fn set_r(builder: UniquePtr<BGVContextBuilder>, r: u32) -> UniquePtr<BGVContextBuilder>;

        fn is_bootstrappable(builder: UniquePtr<BGVContextBuilder>, flag: bool) -> UniquePtr<BGVContextBuilder>;
        fn set_thickboot(builder: UniquePtr<BGVContextBuilder>) -> UniquePtr<BGVContextBuilder>;
        fn set_thinboot(builder: UniquePtr<BGVContextBuilder>) -> UniquePtr<BGVContextBuilder>;

        fn build_ptr(builder: UniquePtr<BGVContextBuilder>) -> UniquePtr<Context>;

        // fn set_gens(builder: UniquePtr<BGVContextBuilder>, gens: &CxxVector<i64>);
        // fn set_mvec(builder: UniquePtr<BGVContextBuilder>, mvec: &CxxVector<i64>);
        // fn set_ords(builder: UniquePtr<BGVContextBuilder>, ords: &CxxVector<i64>);

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
// #[derive(Debug)]
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

    // Note: This consumes the Builder instance when `cb.build()` is called.
    //       That is you won't be able to use `cb` afterward.
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

    fn metric_set(self, metric: Metric) -> Result<Builder, BGVError> {
        // Delegate to the implementation for each field.
        match metric {
            Metric::Bits(value) => self.set_bits(value),
            // Metric::Bootstrap(value) => self.set_bootstrap(value),
            // Metric::Bootstrappable(value) => self.set_bootstrappable(value),
            Metric::C(value) => self.set_c(value),
            // Metric::Gens(value) => self.set_gens(value),
            Metric::M(value) => self.set_m(value),
            // Metric::Mvec(value) => self.set_mvec(value),
            // Metric::Ords(value) => self.set_ords(value),
            Metric::P(value) => self.set_p(value),
            Metric::R(value) => self.set_r(value),
            _ => todo!()
        }
    }
}

impl Setters for Builder {
    fn set_bits<T, E>(mut self, value: T) -> Result<Self, BGVError>
    where
        Self: Sized,
        T: ToU32<E>,
        E: Into<SetError>,
    {
        let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
        // Assuming `ffi::set_bits` returns Result<(), BitsError>
        self.inner = ffi::set_bits(self.inner, u32_value);
        Ok(self)
    }

    fn set_c<T, E>(mut self, value: T) -> Result<Self, BGVError>
    where
        Self: Sized,
        T: ToU32<E>,
        E: Into<SetError>,
    {
        let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
        // Assuming `ffi::set_c` returns Result<(), CError>
        self.inner = ffi::set_c(self.inner, u32_value);
        Ok(self)
    }

    fn set_m<T, E>(mut self, value: T) -> Result<Self, BGVError>
    where
        Self: Sized,
        T: ToU32<E>,
        E: Into<SetError>,
    {
        let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
        // Assuming `ffi::set_m` returns Result<(), MError>
        self.inner = ffi::set_m(self.inner, u32_value);
        Ok(self)
    }

    fn set_p<T, E>(mut self, value: T) -> Result<Self, BGVError>
    where
        Self: Sized,
        T: ToU32<E>,
        E: Into<SetError>,
    {
        let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
        // Assuming `ffi::set_p` returns Result<(), PError>
        self.inner = ffi::set_p(self.inner, u32_value);
        Ok(self)
    }

    fn set_r<T, E>(mut self, value: T) -> Result<Self, BGVError>
    where
        Self: Sized,
        T: ToU32<E>,
        E: Into<SetError>,
    {
        let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
        // Assuming `ffi::set_r` returns Result<(), RError>
        self.inner = ffi::set_r(self.inner, u32_value);
        Ok(self)
    }

    fn set(self, value: Metric) -> Result<Self, BGVError>
    {

        // Convert `value` into `Metric`, since `Into` is infallible
        let metric: Metric = value.into();

        self.metric_set(metric)
    }

    fn try_set<T: TryInto<Metric, Error=BGVError>>(self, value: T) -> Result<Self, BGVError>
    where
        Self: Sized,
    {
        // Convert `value` into `Metric`, since `TryInto` is fallible
        let metric = value.try_into()?;

        self.metric_set(metric)
    }
}


// Primarily test success scenarios as the `try_from(...)` implementations
// handle the error scenarios.
#[cfg(test)]
mod tests {
    use super::*;
    use cxx::CxxVector;

    // Helper function to create a Context with default or dummy data.
    fn setup_bgv_context(parameters: Parameters) -> Context {
        Context::new(parameters).expect("BGV context creation")
    }

    // Test-specific Error types
    #[derive(Debug, PartialEq)]
    struct TestError(String);

    // Assuming BGVError can encapsulate or be constructed from TestError
    impl From<TestError> for BGVError {
        fn from(e: TestError) -> Self {
            match e {
                TestError(s) => BGVError::GenericError(GenericError { kind: GenericErrorKind::Unexpected(s) })
            }
        }
    }

    #[test]
    fn test_set_bits_success() {
        let builder = Builder::new();
        let result = builder.set(Bits::try_from(500).unwrap().into());
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_c_success() {
        let builder = Builder::new();
        let result = builder.set(C::try_from(2).unwrap().into());
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_m_success() {
        let builder = Builder::new();
        let result = builder.set(M::try_from(42).unwrap().into());
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_p_success() {
        let builder = Builder::new();
        let result = builder.set(P::try_from(2).unwrap().into());
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_r_success() {
        let builder = Builder::new();
        let result = builder.set(R::try_from(1).unwrap().into());
        assert!(result.is_ok());
    }

    #[test]
    fn test_context_with_defaults() {
        let context = Context::new(Parameters::default());
        assert!(context.is_ok());
    }

}
