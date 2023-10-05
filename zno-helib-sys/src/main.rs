use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    // C++ headers to include, relative to the path in build/main.rs.
    #include "helib/helib.h"
    safety!(unsafe_ffi) // see details of unsafety policies described in the 'safety' section of the book
    generate_ns!("helib") // add this line for each function or type you wish to generate
}

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
