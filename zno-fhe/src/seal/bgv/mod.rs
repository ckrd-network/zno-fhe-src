// Include the modules
pub mod m;
// pub mod p;
// pub mod r;
// pub mod c;
// pub mod bits;
// pub mod gens;
// pub mod ords;
// pub mod mvec;
// pub mod bootstrap;
// pub mod bootstrappable;
pub mod parameters;
pub mod context;
// pub mod error;
// pub mod getters;
// pub mod setters;
pub mod metric;

// Re-export the types for external use as `crate::bgv::<type>`
pub use self::m::*;
// pub use self::p::*;
// pub use self::r::*;
// pub use self::c::*;
// pub use self::bits::*;
// pub use self::gens::*;
// pub use self::ords::*;
// pub use self::mvec::*;
// pub use self::bootstrap::*;
// pub use self::bootstrappable::*;
// pub use self::parameters::*;
// pub use self::context::*;
// pub use self::error::*;
// pub use self::getters::*;
// pub use self::setters::*;
pub use self::metric::*;

use crate::prelude::*;

/// Enumerates the various homomorphic encryption schemas available.
///
/// Each variant represents a different scheme with its own set of features
/// and trade-offs.
///
/// # Examples
///
/// ```
/// # use your_crate::Schema;
/// let schema = Schema::Bgv;
/// ```
pub enum Schema {
    /// The Brakerski-Gentry-Vaikuntanathan (BGV) scheme.
    Bgv,
    // Bfv, // The Brakerski/Fan-Vercauteren (BFV) scheme.
    // Ckks, // The Cheon-Kim-Kim-Song (CKKS) scheme.
}

impl FheScheme for Schema {}

// Implement the Scheme marker trait for required types
// This enforces implementation of required traits: He, Into<Metric>
// impl Scheme for Bits {}
// impl Scheme for Bootstrap {}
// impl Scheme for Bootstrappable {}
// impl Scheme for C {}
// impl Scheme for Gens {}

// overlapping implementation
// impl <S: FheScheme> Fhe<S> for M
// {
//     fn schema(&self, _schema: Schema) {
//         // Can't do anything here because FheMetric doesn't provide any methods
//         unimplemented!()
//     }

//     fn get_schema(&self) -> S {
//         // Can't do anything here because FheMetric doesn't provide any methods
//         unimplemented!()
//     }
// }
// impl Scheme for Mvec {}
// impl Scheme for Ords {}
// impl Scheme for P {}
// impl Scheme for R {}
