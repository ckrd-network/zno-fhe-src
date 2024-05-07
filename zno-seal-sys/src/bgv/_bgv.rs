use core::pin::Pin;
use core::fmt;
use core::convert::TryFrom;
use core::convert::TryInto;
use std::ptr;
use std::{fmt::{Display, Formatter}, error::Error};

use cxx::CxxString;
use cxx::CxxVector;
use cxx::UniquePtr;

use crate::prelude::*;


impl Builder {
    // Constructs a new Builder
    pub fn new() -> Self {
        Self {
            // Return a UniquePtr to the C++ object
            // UniquePtr<super::ffi::Context>, such that the `build()` method succeeds.
            // In C++ `new` is as reserved keyword, so use `init` instead.
            inner: self::ffi::init(Schema::default()),
        }
    }

    // Note: This consumes the Builder instance when `cb.build()` is called.
    //       That is you won't be able to use `cb` afterward.
    // pub fn build(self) -> Result<Context, FFIError> {
    //     // This is safe because it transfers ownership of the Context
    //     let context_ptr = super::ffi::build(self.inner);
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

// impl Builder {
    // ... other methods

    // pub fn from_cipher_str(&self, str: &str) -> Ciphertext {
    //     let mut cipher = Ciphertext::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // super::ffi::load(&mut cipher, &self.inner, &in_stream);
    //     cipher
    // }

    // pub fn from_plain_str(&self, str: &str) -> Plaintext {
    //     let mut plain = Plaintext::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // super::ffi::load(&mut plain, &self.inner, &in_stream);
    //     plain
    // }

    // pub fn from_secret_str(&self, str: &str) -> SecretKey {
    //     let mut secret = SecretKey::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // super::ffi::load(&mut secret, &self.inner, &in_stream);
    //     secret
    // }

    // pub fn from_public_str(&self, str: &str) -> PublicKey {
    //     let mut public_ = PublicKey::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // super::ffi::load(&mut public_, &self.inner, &in_stream);
    //     public_
    // }

    // pub fn from_relin_str(&self, str: &str) -> RelinKeys {
    //     let mut relin = RelinKeys::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // super::ffi::load(&mut relin, &self.inner, &in_stream);
    //     relin
    // }

    // pub fn from_galois_str(&self, str: &str) -> GaloisKeys {
    //     let mut galois = GaloisKeys::new();
    //     let in_stream = std::stringstream::new(str);
    //     // TODO
    //     // super::ffi::load(&mut galois, &self.inner, &in_stream);
    //     galois
    // }
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
        assert_eq!(super::ffi::Schema::none as u8, 0x0);
        assert_eq!(super::ffi::Schema::bfv as u8, 0x1);
        assert_eq!(super::ffi::Schema::ckks as u8, 0x2);
        assert_eq!(super::ffi::Schema::bgv as u8, 0x3);
        assert_eq!(super::ffi::Schema::default() as u8, 0x3);
    }

    #[test]
    fn test_from_u8_to_ffi() {
        assert_eq!(super::ffi::Schema::from(0x0 as u8), super::ffi::Schema::none);
        assert_eq!(super::ffi::Schema::from(0x1 as u8), super::ffi::Schema::bfv);
        assert_eq!(super::ffi::Schema::from(0x2 as u8), super::ffi::Schema::ckks);
        assert_eq!(super::ffi::Schema::from(0x3 as u8), super::ffi::Schema::bgv);
        assert_eq!(super::ffi::Schema::from(0x4 as u8), super::ffi::Schema::default()); // default
        assert_eq!(super::ffi::Schema::from(0xFF as u8), super::ffi::Schema::default()); // default
    }

    #[test]
    fn test_from_zno_fhe_to_ffi() {
        let schema = Schema::None;
        let ffi_schema = super::ffi::Schema::from(schema);
        assert_eq!(ffi_schema, super::ffi::Schema::none);

        let schema = Schema::Bfv;
        let ffi_schema = super::ffi::Schema::from(schema);
        assert_eq!(ffi_schema, super::ffi::Schema::bfv);

        let schema = Schema::Ckks;
        let ffi_schema = super::ffi::Schema::from(schema);
        assert_eq!(ffi_schema, super::ffi::Schema::ckks);

        let schema = Schema::Bgv;
        let ffi_schema = super::ffi::Schema::from(schema);
        assert_eq!(ffi_schema, super::ffi::Schema::bgv);
    }

    #[test]
    fn test_from_ffi_to_zno_fhe() {
        let ffi_schema = super::ffi::Schema::none;
        let schema = Schema::from(ffi_schema);
        assert_eq!(schema, Schema::None);

        let ffi_schema = super::ffi::Schema::bfv;
        let schema = Schema::from(ffi_schema);
        assert_eq!(schema, Schema::Bfv);

        let ffi_schema = super::ffi::Schema::ckks;
        let schema = Schema::from(ffi_schema);
        assert_eq!(schema, Schema::Ckks);

        let ffi_schema = super::ffi::Schema::bgv;
        let schema = Schema::from(ffi_schema);
        assert_eq!(schema, Schema::Bgv);
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

    #[test]
    fn test_set_m_success() {
        let builder = Builder::new();
        let result = builder.set(M::try_from(42).unwrap().into());
        assert!(result.is_ok());
    }

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
