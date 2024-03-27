use super::super::*;

use std::os::raw::c_int;
use std::pin::Pin;

// This module provides Rust bindings and wrappers for the C++ SEAL library.
//
// The primary aim is to ensure memory safety, proper resource management, and to
// provide an idiomatic Rust interface for the C++ functions and objects.
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("seal/seal.h");
        #[namespace = "seal"]
        type EncryptedArray;
        #[namespace = "seal"]
        type EncryptedArrayConst;
        #[namespace = "seal"]
        type Context;

        // fn toConst(handle: UniquePtr<EncryptedArray>) -> UniquePtr<EncryptedArrayConst>;

        #[namespace = "seal"]
        // fn getHandle(self: &Context, handle: &mut UniquePtr<EncryptedArray>);
        // cxx-bridge recognizes a nullptr value as std::unique_ptr, mapping it to Option::None for UniquePtr<T>.
        fn getHandle(self: Pin<&mut Context>) -> UniquePtr<EncryptedArray>;
        fn getUseCount(self: &Context) -> i64;
    }
}

//  This trait tells the cxx bridge that the Context type is safe to be passed between Rust and C++.
impl cxx::ExternType for Context {
    type Id = cxx::type_id!("seal::Context");
    type Kind = cxx::kind::Trivial;
}

/// Represents a handle to an `EncryptedArray` from the C++ SEAL library.
///
/// Even though this struct wraps a non-const pointer from the C++ side, we do not
/// provide any methods that allow mutations to the underlying data. This is to ensure
/// that the data is used responsibly in Rust and doesn't accidentally modify the
/// underlying `EncryptedArray`.
///
/// The underlying `UniquePtr` to the C++ object is automatically cleaned up when
/// this struct goes out of scope, ensuring proper resource management.
struct EncryptedArrayHandle {
    inner: ffi::UniquePtr<ffi::EncryptedArray>,
}

impl EncryptedArrayHandle {
    // If you need any methods to access data without mutating it, add them here.
}

// Suppress until needed - leads to a cxx error when empty.
// impl Drop for EncryptedArrayHandle {
//     fn drop(&mut self) {
//         // Handle is automatically dropped here.
//     }
// }

/// Represents a const handle to an `EncryptedArray` from the C++ SEAL library.
///
/// The underlying `UniquePtr` to the C++ object is automatically cleaned up when
/// this struct goes out of scope, ensuring proper resource management.
struct EncryptedArrayConstHandle {
    inner: ffi::UniquePtr<ffi::EncryptedArrayConst>,
}

impl EncryptedArrayConstHandle {
    // Any methods specific to the const handle can be added here.
}

// Suppress until needed - leads to a cxx error when empty.
// impl Drop for EncryptedArrayConstHandle {
//     fn drop(&mut self) {
//         // Const handle is automatically dropped here.
//     }
// }

// fn main() {
//     let ctx = ffi::Context::new();

//     // Using getHandle
//     let mut raw_handle = ffi::UniquePtr::null();
//     ctx.getHandle(&mut raw_handle);

//     let handle = EncryptedArrayHandle { inner: raw_handle };

//     // Converting handle to const handle
//     let raw_const_handle = ffi::convertToConst(handle.inner);
//     let const_handle = EncryptedArrayConstHandle { inner: raw_const_handle };

//     // Using getUseCount
//     let count = ctx.getUseCount();
//     println!("Use count: {}", count);
// }

// let maybe_encrypted_array: Option<UniquePtr<EncryptedArray>> = context.transfer_handle();

// match maybe_encrypted_array {
//     Some(encrypted_array) => {
//         // Use the EncryptedArray here
//     },
//     None => {
//         println!("Failed to obtain ownership of EncryptedArray");
//     }
// }