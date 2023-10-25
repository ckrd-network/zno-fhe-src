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
pub mod bgv_params;
pub mod context;
pub mod error;

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
pub use self::bgv_params::*;
pub use self::context::*;
pub use self::error::*;

pub trait ToU32<E> {
    fn to_u32(&self) -> Result<u32, E>;
}
