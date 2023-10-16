use cxx::UniquePtr;
use std::os::raw::c_long;
use cxx::rust::String;
use std::convert::TryInto;
use cxx::CxxString;
use cxx::CxxVector;
use std::ffi::CStr;
// Import the BGV struct and its fields
use super::bgv::*;

#[cxx::bridge(namespace="helib")]
mod ffi {

    unsafe extern "C++" {
        include!("zno-helib-sys/ffi/ffi_wrapper.h");


        // Non-template representation for BGV.
        type Context;
        type BGVContextBuilder;

        // Representations of the C++ ContextBuilder methods in Rust.
        // Assuming the builder methods don't need parameters from Rust.
        // fn build(self: &BGVContextBuilder) -> UniquePtr<Context>;

        // fn makeBGVContextBuilder() -> UniquePtr<BGVContextBuilder>;

        // fn from_std_vector(rustVec: &CxxVector<i64>) -> Vec<i64>;

        fn to_std_vector(rust_vec: &Vec<i64>) -> UniquePtr<CxxVector<i64>>;

        fn new_bgv_builder() -> UniquePtr<BGVContextBuilder>;
        fn set_m(builder: UniquePtr<BGVContextBuilder>, m: i32) -> UniquePtr<BGVContextBuilder>;

        // fn build_ptr(builder: Pin<&mut BGVContextBuilder>) -> UniquePtr<Context>;
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

    // External Rust functions, used to convert Rust types into types that can be used in C++:
    extern "Rust" {
        fn convert_to_vec(s: &str) -> Vec<i64>;
    }
}

// This part should be outside of the `mod ffi` block
unsafe impl cxx::ExternType for ffi::BGVContextBuilder {
    type Id = cxx::type_id!("BGVContextBuilder");
    type Kind = cxx::kind::Trivial;
}

pub fn make_context_builder() -> cxx::UniquePtr<ffi::BGVContextBuilder> {
    ffi::makeBGVContextBuilder()
}

pub fn build_context(builder: Pin<&mut BGVContextBuilder>) -> UniquePtr<ffi::Context> {
    // This call is safe because it transfers ownership of the Context
    // to the UniquePtr, which ensures it will be cleaned up correctly.
    ffi::build_ptr(builder)
}

pub struct BGVContextBuilder {
    // Holds the actual C++ object
    inner: ffi::UniquePtr<ffi::BGVContextBuilder>,
}

impl BGVContextBuilder {
    // Constructs a new BGVContextBuilder
    pub fn new() -> Self {
        BGVContextBuilder {
            inner: ffi::new_bgv_builder(),
        }
    }

    // Sets the `m` parameter and enables method chaining
    pub fn set_m(mut self, m: i32) -> Self {
        self.inner = ffi::set_m(self.inner, m);
        self
    }

    // Similarly, implement other setters following the same pattern...

    // When you're done building
    pub fn build(self) -> ffi::UniquePtr<ffi::BGVContextBuilder> {
        self.inner
    }
}


/// fn main() {
///     let builder = BGVContextBuilder::new()
///         .set_m(123)
///         // ... other settings ...
///         .build(); // Finalize and get the actual C++ object
///
///     // Now you can pass `builder` to other FFI functions expecting a BGVContextBuilder
/// }

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
/// Remember that Rust's safety principles apply, and the UniquePtr<ffi::Context> ensures that when the Context goes out of scope, it will be properly cleaned up (its destructor will be called), preventing memory leaks.
///
/// Define the Rust struct to represent the C++ Context class

struct Context {
    // Fields matching the C++ constructor arguments
    m: c_long,
    p: c_long,
    r: c_long,
    bits: c_long,
    gens: &Vec<c_long>,
    ords: &Vec<c_long>,
}

// Define methods for the Rust Context struct
impl Context {

    pub fn convert_to_vec(s: &str) -> Vec<i64> {
        s.split(',')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect()
    }

    // Create Rust functions to create BGV contexts using FFI
    pub fn create_bgv_context_default() -> UniquePtr<Context> {
        ffi::create_context_default()
    }

    // Define a method to create a Context instance using one constructor
    pub fn create_bgv_context(bgvs: BGV) -> UniquePtr<Context> {
        let gens_vec: Vec<i64> = bgvs.gens.iter().cloned().collect();
        let ords_vec: Vec<i64> = bgvs.ords.iter().cloned().collect();

        let cntxt = ffi::helib::create_bgv_context_wrapper(bgvs.m, bgvs.p, bgvs.r, bgvs.bits, &gens_vec, &ords_vec);
        cntxt
    }

    // pub fn create_bgv_context_params_extended(bgvs: BGV, mparams: Option<ModChainParams>, bparams: Option<BootStrapParams>) -> UniquePtr<Context> {
    //     let gens_vec: Vec<i64> = bgvs.gens.iter().cloned().collect();
    //     let ords_vec: Vec<i64> = bgvs.ords.iter().cloned().collect();

    //     ffi::create_context_params_extended(bgvs.m, bgvs.p, bgvs.r, gens_vec, ords_vec, mparams, bparams)
    // }

    // pub fn create_bgv_context_serializable(content: SerializableContent) -> UniquePtr<Context> {
    //     ffi::create_context_serializable(content)
    // }
}