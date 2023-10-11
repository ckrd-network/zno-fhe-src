#![allow(bad_style, deprecated, clippy::all)]

// use libc::*;
use zno_helib_sys::*;

fn main() {
    println!("The answer to Life, The Universe and Everything is {:#?}", ffi::helib::version::libString());
}

// #[test]
// fn test_bridge_issue_2() {
//     let encrypted_array_instance = ...; // Initialize or get an EncryptedArray object.
//     let encrypted_array_rust = EncryptedArrayRust::new(&encrypted_array_instance);
//     encrypted_array_rust.use_encrypted_array();
//     // For debugging or verification
//     let count = encrypted_array_rust.get_use_count();
//     println!("Use count of EncryptedArray: {}", count);
// }
