pub mod prelude;
pub mod zno;

use autocxx::prelude::*; // use all the main autocxx functions

include_cpp! {
    // C++ headers to include, relative to the path in build/main.rs.
    #include "helib/helib.h"
    safety!(unsafe_ffi) // see details of unsafety policies described in the 'safety' section of the book
    // generate_ns!("helib") // add this line for each function or type you wish to generate
    generate!("helib::Context") // add this line for each function or type you wish to generate
}
