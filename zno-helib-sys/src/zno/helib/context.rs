// use cxx::UniquePtr;

// // Import the BGV struct and its fields
// use your_module::BGV;
// use your_module::{M, P, R, Gens, Ords};

// #[cxx::bridge]
// mod ffi {
//     extern "C++" {
//         include!("helib/helib.h"); // Include the C++ header file

//         // Define the C++ constructor's FFI equivalent
//         fn create_context(m: u64, p: u64, r: u64, gens: Vec<i64>, ords: Vec<i64>) -> UniquePtr<Context>;
//     }
// }

// // Create a Rust function to create a BGV context using FFI
// pub fn create_bgv_context(bgvs: BGV) -> UniquePtr<Context> {
//     let gens_vec: Vec<i64> = bgvs.gens.iter().cloned().collect();
//     let ords_vec: Vec<i64> = bgvs.ords.iter().cloned().collect();

//     ffi::create_context(bgvs.m, bgvs.p, bgvs.r, gens_vec, ords_vec)
// }
