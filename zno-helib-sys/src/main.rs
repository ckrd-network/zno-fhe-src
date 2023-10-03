use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    safety!(unsafe_ffi) // see details of unsafety policies described in the 'safety' section of the book
    generate_ns!("helib") // add this line for each function or type you wish to generate
    // generate!("ContextBuilder")
    // generate!("SecKey")
    // generate!("PubKey")
    // generate!("EncryptedArray")
    // generate!("ContextBuilder")
    // generate!("ContextBuilder")
    // generate!("ContextBuilder")
}