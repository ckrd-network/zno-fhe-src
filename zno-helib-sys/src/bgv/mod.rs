// Include the modules
mod m;
mod p;
mod r;
mod c;
mod bits;
mod gens;
mod ords;
mod mvec;
mod bootstrap;
mod bootstrappable;
mod bgv_params;

// Optionally, if you want to re-export the types for external use
pub use m::M;
pub use p::P;
pub use r::R;
pub use c::C;
pub use bits::Bits;
pub use gens::Gens;
pub use ords::Ords;
pub use mvec::Mvec;
pub use bootstrap::Bootstrap;
pub use bootstrappable::Bootstrappable;
pub use bgv_params::BGVParams;

// This brings `Context` to `bgv` module level, avoiding the need to include `ffi` in the path
#[cfg(feature = "helib")]
pub use crate::helib::bgv::Context;
