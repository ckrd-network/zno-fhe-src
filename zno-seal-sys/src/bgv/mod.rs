pub mod version;

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

    unsafe extern "C++" {
        include!("zno-seal-sys/ffi/ffi_wrapper.h");

        type EncryptionParameters;

        fn get_scheme(params: UniquePtr<EncryptionParameters>) -> u8;
        fn set_scheme(params: UniquePtr<EncryptionParameters>, scheme: u8);

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

        fn init(schema: UniquePtr<CxxU8>) -> UniquePtr<BGVContextBuilder>;

        fn build(builder: UniquePtr<BGVContextBuilder>) -> UniquePtr<Context>;

        fn set_m(builder: UniquePtr<BGVContextBuilder>, m: u32) -> UniquePtr<BGVContextBuilder>;

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

}
