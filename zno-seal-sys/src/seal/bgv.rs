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

#[cxx::bridge(namespace="seal")]
pub mod ffi {

    struct SEALVersion {
        major: u8,
        minor: u8,
        patch: u8,
        tweak: u8,
    }

    unsafe extern "C++" {
        include!("zno-seal-sys/ffi/ffi_wrapper.h");

        type Schema;

        type BGVContextBuilder;
        type Context;
        type Ciphertext;
        type Plaintext;
        type SecretKey;
        type PublicKey;
        type RelinKeys;
        type GaloisKeys;

        type Parameters;
        type SecurityLevel;

        fn version() -> UniquePtr<CxxString>;

        fn init(schema: UniquePtr<Schema>) -> UniquePtr<BGVContextBuilder>;

        fn build(builder: UniquePtr<BGVContextBuilder>) -> UniquePtr<Context>;

        // // Methods of SEALContext
        // fn get_context_data(self: &SEALContext);
        // fn key_context_data(self: &SEALContext);
        // fn first_context_data(self: &SEALContext);
        // fn parameters_set(self: &SEALContext);
        // fn first_parms_id(self: &SEALContext);
        // fn last_parms_id(self: &SEALContext);
        // fn using_keyswitching(self: &SEALContext);

        // Pending implementation...
        //
        // // ... from_*_str function load helper methods for SEALContext.
        // // The `load` functions are member functions of the `Ciphertext`, `Plaintext`, `SecretKey`, `PublicKey`, `RelinKeys`, and `GaloisKeys` classes.
        // fn load(self: Pin<&mut Ciphertext>, context: &SEALContext, in_stream: &String);
        // fn load(self: Pin<&mut Plaintext>, context: &SEALContext, in_stream: &String);
        // fn load(self: Pin<&mut SecretKey>, context: &SEALContext, in_stream: &String);
        // fn load(self: Pin<&mut PublicKey>, context: &SEALContext, in_stream: &String);
        // fn load(self: Pin<&mut RelinKeys>, context: &SEALContext, in_stream: &String);
        // // fn loader(self: Pin<&mut GaloisKeys>, context: &SEALContext, in_stream: &String);
        // fn loader(self: Pin<&mut GaloisKeysWrapper>, context: &SEALContext, in_str: &str) -> i64;
    }

    extern "Rust" {
        type M;
        type MError;
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Schema {
    None = 0x0,
    Bfv = 0x1,
    Ckks = 0x2,
    Bgv = 0x3,
}

/// Implements the `Default` trait for `ffi::Schema`.
/// Returns the default value for `ffi::Schema`, which is `ffi::Schema::bgv`.
impl Default for Schema {
    /// Returns the default value for `ffi::Schema`, which is `ffi::Schema::bgv`.
    ///
    /// # Arguments
    ///
    /// * `self` - The `ffi::Schema` to set to the default value.
    ///
    /// # Returns
    ///
    /// The default value for `ffi::Schema`, which is `ffi::Schema::bgv`.
    fn default() -> Self {
        Schema::Bgv
    }
}

/// Converts a u8 value into a `ffi::Schema` enum variant.
///
/// # Arguments
///
/// * `item` - The u8 value to convert.
///
/// # Returns
///
/// The corresponding `ffi::Schema` enum variant.
///
/// # Examples
///
/// ```
/// use ffi::Schema;
///
/// let value: u8 = 0x1;
/// let schema = Schema::from(value);
/// assert_eq!(schema, Schema::Bfv);
/// ```
impl From<u8> for Schema {
    fn from(item: u8) -> Self {
        match item {
            0x0 => Schema::default(),
            0x1 => Schema::Bfv,
            0x2 => Schema::Ckks,
            0x3 => Schema::Bgv,
            _ => Schema::default(), // default
        }
    }
}

/// Converts an `ffi::Schema` value into a `Schema` enum variant.
///
/// # Arguments
///
/// * `item` - The `ffi::Schema` value to convert.
///
/// # Returns
///
/// The corresponding `Schema` enum variant.
///
/// # Examples
///
/// ```
/// use Schema;
///
/// let ffi_schema = ffi::Schema::bfv;
/// let schema: Schema = ffi_schema.into();
/// assert_eq!(schema, Schema::Bfv);
/// ```

impl From<ffi::Schema> for Schema {
    fn from(item: ffi::Schema) -> Self {
        match item {
            ffi::Schema::bfv => Schema::Bfv,
            ffi::Schema::ckks => Schema::Ckks,
            ffi::Schema::bgv => Schema::Bgv,
            _ => Schema::default(),
        }
    }
}

/// Converts a `Schema` enum variant into an `ffi::Schema` value.
///
/// # Arguments
///
/// * `item` - The `Schema` enum variant to convert.
///
/// # Returns
///
/// The corresponding `ffi::Schema` value.
///
/// # Examples
///
/// ```
/// use Schema;
///
/// let schema = Schema::Bfv;
/// let ffi_schema: ffi::Schema = schema.into();
/// assert_eq!(ffi_schema, ffi::Schema::bfv);
/// ```

impl From<Schema> for ffi::Schema {
    fn from(item: Schema) -> Self {
        match item {
            Schema::Bfv => ffi::Schema::bfv,
            Schema::Ckks => ffi::Schema::ckks,
            Schema::Bgv => ffi::Schema::bgv,
            _ => ffi::Schema::none,
        }
    }
}

/// Converts a `zno_fhe::Schema` into an `crate::Schema`.
///
impl From<zno_fhe::Schema> for Schema {
    /// Converts the given `zno_fhe::Schema` into the corresponding `ffi::Schema` variant.
    ///
    /// # Arguments
    ///
    /// * `schema` - The `zno_fhe::Schema` to convert.
    ///
    /// # Returns
    ///
    /// The converted `ffi::Schema` variant.
    /// The converted `ffi::Schema` variant.
    fn from(schema: zno_fhe::Schema) -> Self {
        match schema {
            zno_fhe::Schema::None => Schema::default(),
            zno_fhe::Schema::Bfv => Schema::Bfv,
            zno_fhe::Schema::Ckks => Schema::Ckks,
            zno_fhe::Schema::Bgv => Schema::Bgv,
        }
    }
}

/// Converts a `Schema` into an `zno_fhe::Schema`.
///
impl From<Schema> for zno_fhe::Schema {
    /// Converts the given `ffi::Schema` into the corresponding `zno_fhe::Schema`.
    ///
    /// # Arguments
    ///
    /// * `schema` - The `ffi::Schema` to convert.
    ///
    /// # Returns
    ///
    /// The converted `zno_fhe::Schema`.
    fn from(schema: Schema) -> Self {
        match schema {
            schema::None => zno_fhe::Schema::default(),
            schema::Bfv  => zno_fhe::Schema::Bfv,
            schema::Ckks => zno_fhe::Schema::Ckks,
            schema::Bgv  => zno_fhe::Schema::Bgv,
        }
    }
}

/// This module contains the definition of the `Builder` struct and its associated methods.
/// The `Builder` struct is responsible for constructing a `SEALContext` object used in the SEAL implementation.
/// It provides methods for setting various parameters and loading data from strings.
/// The `Builder` struct also defines error types for handling null pointer exceptions and other FFI-related errors.
/// The methods of the `Builder` struct are used to configure the parameters of the `SEALContext` object.
/// The `Builder` struct is used in the process of building a `SEALContext` object with the desired parameters.
/// Once the `Builder` is configured, the `build()` method can be called to create a `SEALContext` object.
/// The `Builder` struct is specific to the BGV scheme in the SEAL library.
pub struct Builder {
    // Holds a pointer to the C++ object
    pub inner: cxx::UniquePtr<ffi::SEALContext>,
}

impl Builder {
    // Constructs a new Builder
    pub fn new() -> Self {
        Self {
            // Return a UniquePtr to the C++ object
            // UniquePtr<ffi::Context>, such that the `build()` method succeeds.
            // In C++ `new` is as reserved keyword, so use `init` instead.
            inner: ffi::init(Schema::default()),
        }
    }

    // Note: This consumes the Builder instance when `cb.build()` is called.
    //       That is you won't be able to use `cb` afterward.
    // pub fn build(self) -> Result<Context, FFIError> {
    //     // This is safe because it transfers ownership of the Context
    //     let context_ptr = ffi::build(self.inner);
    //     // Check if the pointer is null
    //     if context_ptr.is_null() {
    //         return Err(FFIError::NullPointer(NullPointerError));
    //     }
    //     Ok(Context { inner: context_ptr })
    // }

    // fn metric_set(self, metric: Metric) -> Result<Builder, BGVError> {
    //     // Delegate to the implementation for each field.
    //     match metric {
    //         Metric::Bits(value) => self.set_bits(value),
    //         // Metric::Bootstrap(value) => self.set_bootstrap(value),
    //         // Metric::Bootstrappable(value) => self.set_bootstrappable(value),
    //         Metric::C(value) => self.set_c(value),
    //         // Metric::Gens(value) => self.set_gens(value),
    //         Metric::M(value) => self.set_m(value),
    //         // Metric::Mvec(value) => self.set_mvec(value),
    //         // Metric::Ords(value) => self.set_ords(value),
    //         Metric::P(value) => self.set_p(value),
    //         Metric::R(value) => self.set_r(value),
    //         _ => todo!()
    //     }
    // }
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

// Logic specific to the SEAL implementation belongs here.
// #[derive(Debug)]
pub struct Builder {
    // Holds a pointer to the C++ object
    pub inner: cxx::UniquePtr<ffi::SEALContext>,
}


impl Builder {
    // ... other methods

    // pub fn from_cipher_str(&self, str: &str) -> Ciphertext {
    //     let mut cipher = Ciphertext::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // ffi::load(&mut cipher, &self.inner, &in_stream);
    //     cipher
    // }

    // pub fn from_plain_str(&self, str: &str) -> Plaintext {
    //     let mut plain = Plaintext::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // ffi::load(&mut plain, &self.inner, &in_stream);
    //     plain
    // }

    // pub fn from_secret_str(&self, str: &str) -> SecretKey {
    //     let mut secret = SecretKey::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // ffi::load(&mut secret, &self.inner, &in_stream);
    //     secret
    // }

    // pub fn from_public_str(&self, str: &str) -> PublicKey {
    //     let mut public_ = PublicKey::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // ffi::load(&mut public_, &self.inner, &in_stream);
    //     public_
    // }

    // pub fn from_relin_str(&self, str: &str) -> RelinKeys {
    //     let mut relin = RelinKeys::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // ffi::load(&mut relin, &self.inner, &in_stream);
    //     relin
    // }

    // pub fn from_galois_str(&self, str: &str) -> GaloisKeys {
    //     let mut galois = GaloisKeys::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // ffi::load(&mut galois, &self.inner, &in_stream);
    //     galois
    // }
}
// impl Setters for Builder {
//     fn set_bits<T, E>(mut self, value: T) -> Result<Self, BGVError>
//     where
//         Self: Sized,
//         T: ToU32<E>,
//         E: Into<SetError>,
//     {
//         let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
//         // Assuming `ffi::set_bits` returns Result<(), BitsError>
//         self.inner = ffi::set_bits(self.inner, u32_value);
//         Ok(self)
//     }

//     fn set_c<T, E>(mut self, value: T) -> Result<Self, BGVError>
//     where
//         Self: Sized,
//         T: ToU32<E>,
//         E: Into<SetError>,
//     {
//         let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
//         // Assuming `ffi::set_c` returns Result<(), CError>
//         self.inner = ffi::set_c(self.inner, u32_value);
//         Ok(self)
//     }

//     fn set_m<T, E>(mut self, value: T) -> Result<Self, BGVError>
//     where
//         Self: Sized,
//         T: ToU32<E>,
//         E: Into<SetError>,
//     {
//         let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
//         // Assuming `ffi::set_m` returns Result<(), MError>
//         self.inner = ffi::set_m(self.inner, u32_value);
//         Ok(self)
//     }

//     fn set_p<T, E>(mut self, value: T) -> Result<Self, BGVError>
//     where
//         Self: Sized,
//         T: ToU32<E>,
//         E: Into<SetError>,
//     {
//         let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
//         // Assuming `ffi::set_p` returns Result<(), PError>
//         self.inner = ffi::set_p(self.inner, u32_value);
//         Ok(self)
//     }

//     fn set_r<T, E>(mut self, value: T) -> Result<Self, BGVError>
//     where
//         Self: Sized,
//         T: ToU32<E>,
//         E: Into<SetError>,
//     {
//         let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
//         // Assuming `ffi::set_r` returns Result<(), RError>
//         self.inner = ffi::set_r(self.inner, u32_value);
//         Ok(self)
//     }

//     fn set(self, value: Metric) -> Result<Self, BGVError>
//     {

//         // Convert `value` into `Metric`, since `Into` is infallible
//         let metric: Metric = value.into();

//         self.metric_set(metric)
//     }

//     fn try_set<T: TryInto<Metric, Error=BGVError>>(self, value: T) -> Result<Self, BGVError>
//     where
//         Self: Sized,
//     {
//         // Convert `value` into `Metric`, since `TryInto` is fallible
//         let metric = value.try_into()?;

//         self.metric_set(metric)
//     }
// }

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
    fn test_from_ffi_to_u8() {
        assert_eq!(ffi::Schema::none as u8, 0x0);
        assert_eq!(ffi::Schema::bfv as u8, 0x1);
        assert_eq!(ffi::Schema::ckks as u8, 0x2);
        assert_eq!(ffi::Schema::bgv as u8, 0x3);
        assert_eq!(ffi::Schema::default() as u8, 0x3);
    }

    #[test]
    fn test_from_u8_to_ffi() {
        assert_eq!(ffi::Schema::from(0x0 as u8), ffi::Schema::none);
        assert_eq!(ffi::Schema::from(0x1 as u8), ffi::Schema::bfv);
        assert_eq!(ffi::Schema::from(0x2 as u8), ffi::Schema::ckks);
        assert_eq!(ffi::Schema::from(0x3 as u8), ffi::Schema::bgv);
        assert_eq!(ffi::Schema::from(0x4 as u8), ffi::Schema::default()); // default
        assert_eq!(ffi::Schema::from(0xFF as u8), ffi::Schema::default()); // default
    }

    #[test]
    fn test_from_zno_fhe_to_ffi() {
        let schema = zno_fhe::Schema::None;
        let ffi_schema = ffi::Schema::from(schema);
        assert_eq!(ffi_schema, ffi::Schema::none);

        let schema = zno_fhe::Schema::Bfv;
        let ffi_schema = ffi::Schema::from(schema);
        assert_eq!(ffi_schema, ffi::Schema::bfv);

        let schema = zno_fhe::Schema::Ckks;
        let ffi_schema = ffi::Schema::from(schema);
        assert_eq!(ffi_schema, ffi::Schema::ckks);

        let schema = zno_fhe::Schema::Bgv;
        let ffi_schema = ffi::Schema::from(schema);
        assert_eq!(ffi_schema, ffi::Schema::bgv);
    }

    #[test]
    fn test_from_ffi_to_zno_fhe() {
        let ffi_schema = ffi::Schema::none;
        let schema = zno_fhe::Schema::from(ffi_schema);
        assert_eq!(schema, zno_fhe::Schema::None);

        let ffi_schema = ffi::Schema::bfv;
        let schema = zno_fhe::Schema::from(ffi_schema);
        assert_eq!(schema, zno_fhe::Schema::Bfv);

        let ffi_schema = ffi::Schema::ckks;
        let schema = zno_fhe::Schema::from(ffi_schema);
        assert_eq!(schema, zno_fhe::Schema::Ckks);

        let ffi_schema = ffi::Schema::bgv;
        let schema = zno_fhe::Schema::from(ffi_schema);
        assert_eq!(schema, zno_fhe::Schema::Bgv);
    }

    // #[test]
    // fn test_set_bits_success() {
    //     let builder = Builder::new();
    //     let result = builder.set(Bits::try_from(500).unwrap().into());
    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn test_set_c_success() {
    //     let builder = Builder::new();
    //     let result = builder.set(C::try_from(2).unwrap().into());
    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn test_set_m_success() {
    //     let builder = Builder::new();
    //     let result = builder.set(M::try_from(42).unwrap().into());
    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn test_set_p_success() {
    //     let builder = Builder::new();
    //     let result = builder.set(P::try_from(2).unwrap().into());
    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn test_set_r_success() {
    //     let builder = Builder::new();
    //     let result = builder.set(R::try_from(1).unwrap().into());
    //     assert!(result.is_ok());
    // }

    #[ignore = "Incomplete SEAL FFI"]
    #[test]
    fn test_context_with_defaults() {
        let context = Context::new(Parameters::default());
        assert!(context.is_ok());
    }

}
