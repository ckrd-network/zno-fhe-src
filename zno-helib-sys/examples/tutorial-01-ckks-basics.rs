extern crate helib;

use std::f64::consts::PI;
use helib::{Context, SecKey, PubKey, PtxtArray, Ctxt};

fn main() {
    let context = Context::new(16 * 1024, 119, 20, 2);

    println!("securityLevel={}", context.security_level());

    let n = context.n_slots();
    let mut secret_key = SecKey::new(&context);
    secret_key.gen_sec_key();
    let public_key = PubKey::from(&secret_key);

    let v0: Vec<f64> = (0..n).map(|i| (2.0 * PI * i as f64 / n as f64).sin()).collect();
    let p0 = PtxtArray::from_vec(&context, &v0);
    let mut c0 = Ctxt::new(&public_key);

    let mut p1 = PtxtArray::random(&context);
    let mut c1 = Ctxt::new(&public_key);
    p1.encrypt(&mut c1);

    let mut p2 = PtxtArray::random(&context);
    let mut c2 = Ctxt::new(&public_key);
    p2.encrypt(&mut c2);

    c0.mul_assign(&c1);
    c2.mul_scalar_assign(1.5);
    c0.add_assign(&c2);

    let mut pp3 = PtxtArray::new(&context);
    pp3.decrypt(&c0, &secret_key);

    let v3: Vec<f64> = pp3.into_vec();

    let mut p3 = p0.clone();
    p3.mul_assign(&p1);
    let mut p4 = p2.clone();
    p4.mul_scalar_assign(1.5);
    p3.add_assign(&p4);

    let distance = p3.distance(&pp3);
    println!("distance={}", distance);
}

// # Rust Type-Driven Design Best Practices Checklist

// ## Requirements Review:

// ### Requirements Description:
// - [x] **R1.1** Clearly understood goal or problem statement about designing efficient Rust types for synchronous-only codebases.
// - [x] **R1.2** Ensure the code is suitable for server platforms focusing on runtime speed and safety.

// ### Rust Specifics:
// - [x] **R2.1** Use the MSRV Rust edition 2021.
// - [x] **R2.2** All dependencies should be sourced from [crates.io](https://crates.io/), ensuring they are widely recognized and well-maintained.

// ## Type Tightness and Design:

// ### Parsing over Validation:
// - [x] **R3.1.1** Adopt thorough parsing methods for inputs to generate structured outputs.
// - [ ] **R3.1.2** Minimize validation, ensuring only correctness checks without structured output generation.

// ### Static Type Systems:
// - [x] **R3.2.1** Leverage Rust's type system for ensuring data safety and correctness.
// - [x] **R3.2.2** A function's type signature should make its purpose and constraints clear.

// ### Avoidance of Partial Functions:
// - [x] **R3.3.1** Always design total functions.
// - [ ] **R3.3.2** Use the type system to indicate unimplementable functions.

// ### Managing Expectations:
// - [x] **R3.4.1** Favor specific types over generic ones.
// - [x] **R3.4.2** Ensure type refinement based on their guarantees.

// ### Prevention of Redundant Checks:
// - [x] **R3.5.1** Function argument types should eliminate redundant downstream checks.
// - [x] **R3.5.2** Post initial checks or parsing, rely on the type system for assurances.

// ### Knowledge Preservation:
// - [ ] **R3.6.1** Types should validate and encapsulate knowledge from validation.
// - [ ] **R3.6.2** Example: Ensure a `NonEmpty` list type is in use where appropriate.

// ### Shotgun Parsing Avoidance:
// - [x] **R3.7.1** Ensure no mixing of input validation and processing code.
// - [x] **R3.7.2** Always stratify programs into clear phases like parsing and execution.

// ### Error Management:
// - [x] **R3.8.1** Errors during parsing shouldn't influence program state or other valid inputs.

// ### Focus on Parsing:
// - [x] **R3.9.1** All data should be parsed before any kind of processing.
// - [x] **R3.9.2** Ensure you handle invalid data cases gracefully.

// ## Style:

// ### Documentation:
// - [ ] **R4.1.1** Every public item should have comprehensive documentation.
// - [ ] **R4.1.2** Provide real-world examples for more complex types or functions.

// ### Testing:
// - [ ] **R4.2.1** Ensure a unit test suite is present.
// - [ ] **R4.2.2** Prefer property-based testing over example-based testing.
