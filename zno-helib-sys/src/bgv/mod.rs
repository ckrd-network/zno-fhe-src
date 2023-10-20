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

// This brings `Context` to `bgv` module level, avoiding the need to include `ffi` in the path
#[cfg(feature = "helib")]
pub use crate::helib::bgv::Context;
#[cfg(feature = "openfhe")]
pub use crate::openfhe::bgv::Context;
#[cfg(feature = "seal")]
pub use crate::seal::bgv::Context;

pub trait ToU32<E> {
    fn to_u32(&self) -> Result<u32, E>;
}

#[derive(Debug, Clone)]
pub enum BGVError {
    BitsError(BitsError),
    BootstrapError(BootstrapError),
    BootstrappableError(BootstrappableError),
    CError(CError),
    GensError(GensError),
    MError(MError),
    MvecError(MvecError),
    OrdsError(OrdsError),
    PError(PError),
    RError(RError),
}

impl From<BitsError> for BGVError {
    fn from(err: BitsError) -> BGVError {
        BGVError::BitsError(err)
    }
}

impl From<BootstrapError> for BGVError {
    fn from(err: BootstrapError) -> BGVError {
        BGVError::BootstrapError(err)
    }
}

impl From<BootstrappableError> for BGVError {
    fn from(err: BootstrappableError) -> BGVError {
        BGVError::BootstrappableError(err)
    }
}

impl From<CError> for BGVError {
    fn from(err: CError) -> BGVError {
        BGVError::CError(err)
    }
}

impl From<GensError> for BGVError {
    fn from(err: GensError) -> BGVError {
        BGVError::GensError(err)
    }
}

impl From<MError> for BGVError {
    fn from(err: MError) -> BGVError {
        BGVError::MError(err)
    }
}

impl From<MvecError> for BGVError {
    fn from(err: MvecError) -> BGVError {
        BGVError::MvecError(err)
    }
}

impl From<OrdsError> for BGVError {
    fn from(err: OrdsError) -> BGVError {
        BGVError::OrdsError(err)
    }
}

impl From<PError> for BGVError {
    fn from(err: PError) -> BGVError {
        BGVError::PError(err)
    }
}

impl From<RError> for BGVError {
    fn from(err: RError) -> BGVError {
        BGVError::RError(err)
    }
}
