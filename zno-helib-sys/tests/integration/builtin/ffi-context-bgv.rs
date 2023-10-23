use zno_helib_sys::prelude::*;
use cxx::UniquePtr;

pub fn setup_ld_library_path() {
    // Determine the directory of the Cargo.toml manifest.
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));

    // Construct the path to the `./libs` directory relative to the manifest directory.
    let libs_path = manifest_dir.join("libs");

    // Convert the path to a string, and panic if it can't be converted.
    // This might happen if the path contains invalid unicode data.
    let libs_str = libs_path.to_str().expect("Invalid unicode in path");

    // Get the current LD_LIBRARY_PATH, if it's set.
    let current_ld_library_path = std::env::var("LD_LIBRARY_PATH").unwrap_or_else(|_| String::new());

    // Prepare the new LD_LIBRARY_PATH to be set.
    let new_ld_library_path = if current_ld_library_path.is_empty() {
        libs_str.to_string()
    } else {
        format!("{}:{}", current_ld_library_path, libs_str)
    };

    // Set the LD_LIBRARY_PATH to include the `./libs` directory.
    // This will only affect child processes started by this process.
    std::env::set_var("LD_LIBRARY_PATH", &new_ld_library_path);

    println!("LD_LIBRARY_PATH has been set to: {}", libs_str);
}

#[test]
fn test_build_bgv_valid_params() {

    setup_ld_library_path();

    let bgv = BGVParams {
        m: M::new(4095).unwrap(),
        p: P::new(2).unwrap(),
        r: R::new(1).unwrap(),
        c: C::new(2).unwrap(),
        bits: Bits::new(32).unwrap(),
        gens: "2,3,5".parse::<Gens>().unwrap(),
        ords: "1,1,1".parse::<Ords>().unwrap(),
        mvec: "2,3".parse::<Mvec>().unwrap(),
        bootstrap: Bootstrap::new("thin").unwrap(),
        bootstrappable: Bootstrappable::new("none").unwrap(),
    };

    let expected = M::new(4095).unwrap();
    let context = Context::new(bgv).expect("BGV context creation");

    // Perform assertions based on what you know about a correctly built Context
    // For instance, you might be able to call functions on `context` to query its state
    // These functions would also need to be exposed via your FFI
    // For example:
    let actual = context.get_m().unwrap(); // this will panic if get_m() returns an Err

    assert_eq!(actual, expected);
    // assert_eq!(context.get_p(), expected_p);
    // ...
    // assert!(context.bootstrappable());

}

// #[test]
// fn test_build_bgv_invalid_params() {
//     let mut builder = ffi::create_context_builder(); // function from your FFI
//     let bgv = BGVParams {
//         // initialize with invalid test values that should cause the build to fail
//         // ...
//     };
//     // Depending on how your FFI and library are set up, you might need to test for
//     // errors or panics here. This could involve `Result`, `Option`, or checking the state
//     // of the returned `Context`, if it's even returned at all.
//     // For example, if your function returns a Result:
//     // assert!(matches!(build_bgv(&mut builder, bgv), Err(_)));
// }

// #[test]
// fn test_create_bgv_context() {
//     // Initialize any resources required for the test
//     // ...
//     // Prepare the arguments for the create_bgv_context function
//     let m: u64 = 4095;
//     let p: u64 = 2;
//     let r: u64 = 1;
//     let gens: Vec<i64> = vec![2, 3, 5];
//     let ords: Vec<i64> = vec![-1, -1, -1];
//     // Call the FFI function
//     let mut builder = ffi::create_context_builder();
//     let bgv = BGVParams {
//         m: M::new(4095).unwrap(),
//         p: P::new(2).unwrap(),
//         r: R::new(1).unwrap(),
//         c: C::new(2).unwrap(),
//         bits: Bits::new("32").unwrap(),
//         gens: "2,3,5".parse::<Gens>().unwrap(),
//         ords: "-1,-1,-1".parse::<Ords>().unwrap(),
//         mvec: "2,3".parse::<Mvec>().unwrap(),
//         bootstrappable: Bootstrappable::new(true),
//     };
//     let context: UniquePtr<ffi::Context> = build_bgv(&mut builder, bgv);
//     let mut builder = ffi::create_context_builder();
//
//     ffi::set_m(&mut builder, params_opts.m.try_into().unwrap());
//     ffi::set_p(&mut builder, p.try_into().unwrap());
//     ffi::set_r(&mut builder, params_opts.r.try_into().unwrap());
//     // ... other settings
//
//     // For vectors, convert them to CxxVector
//     let gens = params_opts.gens.iter().map(|&x| x.try_into().unwrap()).collect::<cxx::CxxVector<_>>();
//     let ords = params_opts.ords.iter().map(|&x| x.try_into().unwrap()).collect::<cxx::CxxVector<_>>();
//
//     ffi::set_gens(&mut builder, &gens);
//     ffi::set_ords(&mut builder, &ords);
//
//     // Assert the results. Here you might need to call other FFI functions to inspect the `Context` object,
//     // since we cannot directly inspect it from Rust.
//     assert_eq!(ffi::context_get_m(&context), 4095);
//     assert!(ffi::context_is_bootstrappable(&context));
//
//     // Clean up any resources you've initialized during the tests
//     // ...
// }
