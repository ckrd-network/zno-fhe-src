use super::bits::BitsError;
use super::bootstrap::BootstrapError;
use super::bootstrappable::BootstrappableError;
use super::c::CError;
use super::context::ConstructionError;
use super::gens::GensError;
use super::m::MError;
use super::mvec::MvecError;
use super::ords::OrdsError;
use super::p::PError;
use super::r::RError;

#[derive(Debug, Clone)]
pub enum BGVError {
    BitsError(BitsError),
    BootstrapError(BootstrapError),
    BootstrappableError(BootstrappableError),
    CError(CError),
    ConstructionError(ConstructionError),
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

impl From<ConstructionError> for BGVError {
    fn from(err: ConstructionError) -> BGVError {
        BGVError::ConstructionError(err)
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
