// Include the modules
pub mod m;
pub mod p;
pub mod r;
pub mod c;
pub mod bits;
pub mod gens;
pub mod ords;
pub mod mvec;
pub mod bootstrap;
pub mod bootstrappable;
pub mod parameters;
pub mod context;
pub mod error;
pub mod getters;
pub mod setters;

// Re-export the types for external use as `crate::bgv::<type>`
pub use self::m::*;
pub use self::p::*;
pub use self::r::*;
pub use self::c::*;
pub use self::bits::*;
pub use self::gens::*;
pub use self::ords::*;
pub use self::mvec::*;
pub use self::bootstrap::*;
pub use self::bootstrappable::*;
pub use self::parameters::*;
pub use self::context::*;
pub use self::error::*;
pub use self::getters::*;
pub use self::setters::*;

pub trait ToU32<E> {
    fn to_u32(&self) -> Result<u32, E>;
}

// trait ContextBarred {}
// trait BuilderBarred {}

// impl ContextBarred for Context {}
// impl BuilderBarred for Builder {}

// impl<T: ContextBarred> Setters for T {
//     // Intentionally empty.
//     // Generates a compile-time error on implementing `Setters` for `Context`.
// }

// impl<T: BuilderBarred> Getters for T {
//     // Intentionally empty.
//     // Generates a compile-time error on implementing `Getters` for `Builder`.
// }