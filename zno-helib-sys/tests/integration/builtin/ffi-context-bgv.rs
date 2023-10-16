use zno_helib_sys::prelude::*;
use cxx::UniquePtr;

#[test]
fn test_build_bgv_valid_params() {
    let mut builder = ffi::new_bgv_builder(); // function from your FFI
    let bgv = BGVParams {
        m: M::new(4095).unwrap(),
        p: P::new(2).unwrap(),
        r: R::new(1).unwrap(),
        c: C::new(2).unwrap(),
        bits: Bits::new("32").unwrap(),
        gens: "2,3,5".parse::<Gens>().unwrap(),
        ords: "-1,-1,-1".parse::<Ords>().unwrap(),
        mvec: "2,3".parse::<Mvec>().unwrap(),
        bootstrappable: Bootstrappable::new(true),
    };

    let context = build_bgv(&mut builder, bgv);

    // Perform assertions based on what you know about a correctly built Context
    // For instance, you might be able to call functions on `context` to query its state
    // These functions would also need to be exposed via your FFI
    // For example:
    // assert_eq!(context.get_m(), expected_m);
    // assert_eq!(context.get_p(), expected_p);
    // ...
    assert!(context.is_bootstrappable());

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
