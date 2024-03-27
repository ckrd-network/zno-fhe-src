// Make the prelude module available at the top level of the crate.
pub mod prelude;

#[cfg(feature = "helib")]
mod helib;

#[cfg(feature = "seal")]
mod seal;

pub mod context;
pub mod error;
pub mod metric;
pub mod schema;

pub use self::context::*;
pub use self::error::*;
pub use self::metric::*;
pub use self::schema::*;

// Re-export the types for external use as `crate::bgv::<type>`
// pub use self::m::*;
// pub use self::p::*;
// pub use self::r::*;
// pub use self::c::*;
// pub use self::bits::*;
// pub use self::gens::*;
// pub use self::ords::*;
// pub use self::mvec::*;
// pub use self::bootstrap::*;
// pub use self::bootstrappable::*;
#[cfg(feature = "helib")]
pub use self::helib::context::*;
#[cfg(feature = "helib")]
pub use self::helib::getters::*;
#[cfg(feature = "helib")]
pub use self::helib::setters::*;
#[cfg(feature = "helib")]
pub use self::helib::parameters::*;
#[cfg(feature = "helib")]
pub use self::helib::schema::*;

#[cfg(feature = "seal")]
pub use self::seal::context::*;
#[cfg(feature = "seal")]
pub use self::seal::getters::*;
#[cfg(feature = "seal")]
pub use self::seal::setters::*;
#[cfg(feature = "seal")]
pub use self::seal::parameters::*;
#[cfg(feature = "seal")]
pub use self::seal::schema::*;

// use zno::FheState;

use std::sync::{Arc, Mutex};

// pub enum FheState {
//     Builder(Builder),
//     Initialized(Initialized),
//     Raw(Raw),
//     Encrypted(Encrypted),
//     Decrypted(Decrypted),
// }

// Marker trait for the generic constraint on types implmenting
// the `Encrypted` and `Decrypted` traits.
//
// trait Fhe {}

// enum FheState<S, B: Builder<S>> {
//     Builder(B),
//     Initialized(S),
//     Raw(S),
//     Encrypted(Encrypted),
//     Decrypted(Decrypted),
// }
// pub struct PublicKey {
//     // fields for the public key
// }

// pub struct SecretKey {
//     // fields for the secret key
// }

// // Binding to Sized means borrowing is not possible.
// // But that's OK because the point is to prevent borrowing in the first place.

// trait Builder<S>: Sized {
//     fn init(self, scheme: Scheme) -> FheState<S, Self>;

//     fn build(self) -> Option<S>;
// }

// // Binding to Sized means borrowing is not possible.
// // But that's OK because the point is to prevent borrowing.
// // A trait that zno-macro-derive module will automatically implement.
// trait Encryptable<S, D: Decrypted<S, Self>>: Sized {

//     fn encrypt(self, scheme: S) -> Vault<S, D, Self>;
// }

// // Binding to Sized means borrowing is not possible.
// // But that's OK because the point is to prevent borrowing.
// // A trait that zno-macro-derive module will automatically implement.
// trait Decryptable<S, E: Encrypted<S, Self>>: Sized {

//     fn decrypt(self, scheme: S) -> Vault<S, Self, E>;
// }

pub trait ToU32<E> {
    fn to_u32(&self) -> Result<u32, E>;
}

enum Vault<S, D: Decryptable<S, E>, E: Encryptable<S, D>> {
    Encryptable(S,D),
    Decryptable(S,E),
}

enum Error {
    NotDone
}

/// A trait for types that represent a homomorphic encryption scheme
/// and can be converted into a `Metric`.
///
/// Types implementing `Scheme` can be used with structures or functions
/// expecting a homomorphic encryption context, and they can also be
/// converted into a `Metric` to provide a standardized interface for
/// metrics or statistics collection.
///
/// # Examples
///
/// Assuming `Bgv` is a struct representing the BGV scheme and `Metric` is
/// an enum that can represent various aspects of schemes:
///
/// ```
/// # use crate::{He, Scheme, Schema, Metric};
/// struct Bgv;
///
/// impl He for Bgv {
///     fn schema(&self) -> Schema {
///         Schema::Bgv
///     }
/// }
///
/// // Assume Metric has a variant for BGV
/// impl From<Bgv> for Metric {
///     fn from(_: Bgv) -> Self {
///         Metric::BgvMetrics { /* ... */ }
///     }
/// }
///
/// impl Scheme for Bgv {}
///
/// // Now Bgv can be used wherever Scheme is required
/// ```
///
/// # Errors
///
/// Implementations should ensure that the conversion to `Metric` via
/// `Into<Metric>` does not fail. If a failure is possible, implement
/// `TryInto` trait and provide details in the implementation
/// documentation.
pub trait Scheme: He + Into<Metric> {}

// // FHE scheme trait
// trait Scheme {}
// // BGV scheme type
// struct Bgv;

// impl Scheme for Bgv {}
// struct BgvBuilder;

// impl Builder<Bgv> for BgvBuilder {

//     fn init(self, scheme: Scheme) -> FheState<Bgv, Self> {
//         // commence initializing the BGV scheme...
//         // ... then move to the next state
//         FheState::Initialized(Bgv)
//     }

//     fn build(self) -> FheState<Bgv, Self> {
//         // complete initializing the BGV scheme...
//         // ... then move to the next state
//         Vault::Decryptable(scheme, Raw)
//         // Some(Bgv)
//     }
// }

// fn test() {
//     #[zno( scheme=bgv, security=128, batch=true, unencrypted_multiply=false,
//         beta=1024, key_switching=hybrid-rns, library=helib,
//         cyclotomic_order=1024, /* alt to security */
//         model=base, multiplications=2, key_omega=3, rotations=0,
//         distribution=ternary, summations=1, plaintext_modulus=0 )]
//     let raw = vec![0,1,2,3];
//     // The above two lines are equivalent to:
//     //
//     //      #[cfg(debug_assertions)]
//     //      let raw = vec![0,1,2,3];
//     //
//     //      #[cfg(not(debug_assertions))]
//     //      fn zno_init() -> zno::Raw {
//     //          let m = zno::bgv::M::new(1);
//     //          let s = zno::bgv::S::new(2);
//     //          let parameters = zno::Parameters::new(zno::Bgv)
//     //                              .from(m)
//     //                              .from(s);
//     //          let context = Arc::new(Mutex::new(zno::Context::new(parameters)));
//     //          let builder = zno::BgvBuilder { context };
//     //          let engine = zno::build(zno::Builder(builder));
//     //          zno::Raw::new(engine, vec![0,1,2,3])
//     //      }
//     //
//     //      #[cfg(not(debug_assertions))]
//     //      let raw: zno::Raw = zno_init();

//     let enc = raw.encrypt();
//     let dec = enc.decrypt();
//     let raw = dec.raw();
//     assert_eq!(vec![0,1,2,3], raw);
// }

// fn build<S, B: Builder<S>>(target: FheState<S, B>) -> Result<S, Error> {
//     // in production, `scheme` comes from an environment variable, proc-macro, configuration file, etc.
//     let scheme = Some(Bgv);

//     match target {
//         FheState::Builder(builder) => {
//             if let Some(s) = scheme {
//                 build(builder.init(s))
//             } else {
//                 Ok(builder.done().ok_or(Error::NotDone)?)
//             }
//         }
//         FheState::Initialized(scheme) => Ok(scheme),
//         FheState::Raw(_) => todo!(),
//         FheState::Decrypted(_) => todo!(),
//         FheState::Encrypted(_) => todo!(),
//     }
// }

// pub struct Rawtext {
//     // fields for the rawtext
// }

// pub struct Decryptedtext {
//     // fields for the decryptedtext
// }

// pub struct Encryptedtext {
//     // fields for the encryptedtext
// }

// // The raw state
// pub struct Raw {
//     rawtext: Rawtext,
// }

// // The initialized state
// // context: Arc<Mutex<Context>>,
// pub struct Initialized {
//     rawtext: Rawtext,
// }


// // The encrypted state
// pub struct Encrypted {
//     encryptedtext: Encryptedtext,
// }

// // The decrypted state
// pub struct Decrypted {
//     decryptedtext: Decryptedtext,
// }

// Transitions:
//    Raw       -> Initialized
//    Initialized -> Encrypted
//    Encrypted -> Decrypted
//    Decrypted -> Encrypted

// Raw is the initial state, so create a function to make a new Raw.
// All usage must go through this function first, which kicks off the entire process.
// Here the FHE context is initialized, encryption keys created, etc.

// impl Raw {
//     pub fn new(rawtext: Rawtext) -> Self {
//         Self { rawtext }
//     }

//     // the encrypt transition function
//     pub fn encrypt(self, public_key: &PublicKey) -> Encrypted {
//         // encryption logic here
//         let encryptedtext = Encryptedtext {
//             // fields for the Encryptedtext
//         };
//         Encrypted { encryptedtext }
//     }
// }

// impl Decryptable<S, D: Decrypted<S, Self>> for Encrypted {
//     // pub fn new(decryptedtext: Decryptedtext) -> Self {
//     //     Self { decryptedtext }
//     // }

//     // The encrypt transition function.
//     // Move from the Decrypted state to the Encrypted state.
//     fn decrypt(self, public_key: &PublicKey) -> D {
//         // encryption logic here
//         let decryptedtext = Decryptedtext {
//             // fields for the decryptedtext
//         };
//         D { decryptedtext }
//     }
// }

// impl<S, D: Decryptable<S, E>, E: Encryptable<S, D>> Decryptable<S, E> for Vault<S, D, E> {
//     fn decrypt(self, scheme: S) -> Vault<S, D, E> {
//         self.decrypt(scheme)
//     }
// }

// impl Encryptable<S, E: Encrypted<S, Self>> for Decrypted {
//     // The decrypt transition function.
//     // Move from the Encrypted state to the Decrypted state.
//     fn encrypt(self, secret_key: &SecretKey) -> Vault<S, E> {
//         // decryption logic here
//         let encryptedtext = Encryptedtext {
//             // fields for the Encryptedtext
//         };
//         E { encryptedtext }
//     }
// }
