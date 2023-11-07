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
pub mod metric;

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
pub use self::metric::*;

pub trait ToU32<E> {
    fn to_u32(&self) -> Result<u32, E>;
}

// Homomorphic encryption marker trait
pub trait He {
    fn schema(&self) -> Schema;
}

pub enum Schema {
    Bgv,
    // Bfv,
    // Ckks,
}

// Define a trait for Homomorphic Encryption Schemes
// A trait that requires both He and Into<Metric>
// Implementing the Scheme trait, requires implementing traits:
// - He
// - Into<Metric>
pub trait Scheme: He + Into<Metric> {}

// Implement the Scheme marker trait for required types
// This enforces implementation of required traits: He, Into<Metric>
// impl Scheme for Bits {}
// impl Scheme for Bootstrap {}
// impl Scheme for Bootstrappable {}
// impl Scheme for C {}
// impl Scheme for Gens {}
impl Scheme for M {}
// impl Scheme for Mvec {}
// impl Scheme for Ords {}
// impl Scheme for P {}
// impl Scheme for R {}
