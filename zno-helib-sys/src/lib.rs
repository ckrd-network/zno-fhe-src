pub mod prelude;
mod zno;

// use autocxx::prelude::*; // use all the main autocxx functions

// include_cpp! {
//     // C++ headers to include, relative to the path in build/main.rs.
//     #include "helib/helib.h"
//     #include "helib/ArgMap.h"
//     safety!(unsafe_ffi) // see details of unsafety policies described in the 'safety' section of the book
//     // generate_ns!("helib") // add this line for each function or type you wish to generate
//     // generate!("helib::Context") // zno-helib-sys/src/cxx-bridge/context.rs
//     generate!("helib::ArgMap") // zno-helib-sys/src/cxx-bridge/arg_map.rs
// }
