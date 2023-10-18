use super::m::M;
use super::p::P;
use super::r::R;
use super::c::C;
use super::bits::Bits;
use super::gens::Gens;
use super::ords::Ords;
use super::mvec::Mvec;
use super::bootstrap::Bootstrap;
use super::bootstrappable::Bootstrappable;
use core::fmt;

#[cfg(not(any(feature = "helib", feature = "openfhe", feature = "seal")))]
compile_error!("You must enable one of the features: `helib` or `openfhe` or `seal`");

/// Represents the complete set of BGV parameters as used by HElib.
///
/// The BGV encryption scheme is versatile, with various parameters affecting its
/// efficiency, noise growth, and versatility in homomorphic computations. When setting
/// up an encryption context in HElib for the BGV scheme, the parameters have to be chosen
/// judiciously to strike a balance between performance and functionality.
///
/// This struct provides a convenient way to manage all these parameters, ensuring they
/// are all present when initializing the BGV scheme in HElib through FFI.
///
/// # Errors
///
/// While the `BGVParams` struct aggregates various parameters and does not directly produce errors,
/// its individual components can. When initializing or working with `BGVParams`, the following errors
/// might be encountered from its components:
///
/// - **M**:
///   - `M::new(u32)`: Can produce an error if the provided value is `0`.
///
/// - **P**:
///   - `P::new(u32)`: Can error if the provided value is negative or if the value is not prime.
///
/// - **R**:
///   - No specific errors. This type just represents a non-negative power of the plaintext space modulus `p`.
///
/// - **C**:
///   - `C::new(u32)`: Can error if the provided value is `0`.
///
/// - **Bits**:
///   - `Bits::new(String)`: Can error if the provided string is not a valid binary representation
///     or if it's not a valid u32 representation.
///
/// - **Gens**:
///   - `Gens::new(Vec<u32>)`: Can error if any value in the vector is `0` or if there are more than
///     three values in the vector.
///
/// - **Ords**:
///   - `Ords::new(Vec<i32>)`: Can error if the provided string is not a valid comma-separated list
///     of i32 numbers or if there are more than three values in the vector.
///
/// - **Mvec**:
///   - `Mvec::new(Vec<u32>)`: Can error if any value in the vector is `0`.
///
/// - **Bootstrappable**:
///   - No specific errors. This type just represents a boolean flag indicating if bootstrapping
///     is enabled or not.
///
/// It's essential to handle these errors gracefully, especially when initializing the `BGVParams` struct
/// from user input or external data sources.
///
/// # Safety
///
/// While the Rust code ensures memory safety and type safety, there are a few points to consider:
/// - When interfacing with external systems (like FFI), ensure that the provided values are within
///   valid bounds and adhere to the constraints of the BGV scheme in HElib.
/// - Invalid parameters might not compromise memory safety but could degrade cryptographic security or system efficiency.
///
/// # Panics
///
/// This struct itself does not panic. However, components like `M`, `P`, etc. might have associated methods
/// that can panic under certain conditions. For instance, using `unwrap()` on a `Result` that contains an
/// error will cause a panic. It's recommended to handle errors gracefully using pattern matching or methods
/// like `is_ok()` and `is_err()` before unwrapping.
///
/// # Lifetimes
///
/// The `BGVParams` struct does not handle references, so there are no explicit lifetimes associated with it.
/// All contained data has `'static` lifetime unless the parameters are created with references, which is
/// not the case in the provided implementations.
///


/// BGVParams encapsulates all the parameters required for the BGV scheme in HElib.
///
/// The BGV encryption scheme is versatile, with various parameters affecting its
/// efficiency, noise growth, and versatility in homomorphic computations. When setting
/// up an encryption context in HElib for the BGV scheme, the parameters have to be chosen
/// judiciously to strike a balance between performance and functionality.
///
/// This struct provides a convenient way to manage all these parameters, ensuring they
/// are all present when initializing the BGV scheme in HElib through FFI.
///
/// # Example
///
/// ```
/// # use zno::bgv::{M, P, R, C, Bits, Gens, Ords, Mvec, Bootstrappable};
/// let params = BGVParams {
///     m: M::new(4095).unwrap(),
///     p: P::new(2).unwrap(),
///     r: R::new(1).unwrap(),
///     c: C::new(2).unwrap(),
///     bits: Bits::new("32").unwrap(),
///     gens: "2,3,5".parse::<Gens>().unwrap(),
///     ords: "-1,-1,-1".parse::<Ords>().unwrap(),
///     mvec: "2,3".parse::<Mvec>().unwrap(),
///     bootstrappable: Bootstrappable::new(true),
/// };
/// ```

use cxx::ExternType;

#[repr(C)]
pub struct BGVParams {
    pub m: M,
    pub p: P,
    pub r: R,
    pub c: C,
    pub bits: Bits,
    pub gens: Gens,
    pub ords: Ords,
    pub mvec: Mvec,
    pub bootstrap: Bootstrap,
    pub bootstrappable: Bootstrappable,
}

impl core::fmt::Display for BGVParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BGVParams(m={}, p={}, r={}, c={}, bits={}, gens={}, ords={}, mvec={}, bootstrappable={})",
               self.m, self.p, self.r, self.c, self.bits, self.gens, self.ords, self.mvec, self.bootstrappable)
    }
}

impl BGVParams {
    #[cfg(feature = "helib")]
    pub fn context(&mut self) -> crate::bgv::Context {
        // Note: the `inner` attribute contains cxx::UniquePtr<crate::helib::bgv::ffi::Context>
        crate::helib::bgv::Context::new(self)
    }
    #[cfg(feature = "openfhe")]
    pub fn context(&self) -> crate::bgv::Context {
        todo!();
        // Note: the `inner` attribute contains cxx::UniquePtr<crate::openfhe::Context>
        crate::openfhe::Context::new(self)
    }
    #[cfg(feature = "seal")]
    pub fn context(&self) -> crate::bgv::Context {
        todo!();
        // Note: the `inner` attribute contains cxx::UniquePtr<crate::seal::Context>
        crate::seal::Context::new(self)
    }
}

// {
//   "m": 4096,
//   "p": 2,
//   "r": 1,
//   "c": 2,
//   "bits": 300,
//   "bootstrap": "none"
// }
// { "m": 4096, "p": 2, "r": 1, "c": 2, "bits": 300, "bootstrap": "none" }
// {"m":4096,"p":2,"r":1,"c":2,"bits":300,"bootstrap":"none"}
