use core::fmt;

// use super::bits::BitsError;
// use super::bootstrap::BootstrapError;
// use super::bootstrappable::BootstrappableError;
// use super::c::CError;
use super::context::ConstructionError;
// use super::gens::GensError;
use super::m::{MError, MErrorKind};
// use super::mvec::MvecError;
// use super::ords::OrdsError;
// use super::p::PError;
// use super::r::RError;

#[derive(Debug, Clone, PartialEq)]
pub enum BGVError {
    // BitsError(BitsError),
    // BootstrapError(BootstrapError),
    // BootstrappableError(BootstrappableError),
    // CError(CError),
    ConstructionError(ConstructionError),
    ConversionError {
        from: &'static str,
        to: &'static str,
        reason: String,
    },
    GenericError(GenericError),
    // GensError(GensError),
    MError(MError),
    // MvecError(MvecError),
    // OrdsError(OrdsError),
    // PError(PError),
    // RError(RError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericError {
    pub(crate) kind: GenericErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum GenericErrorKind {
    Unexpected(String),
    NotImplemented,
    InvalidInput(String),
    // You can add more generic error kinds here.
}

impl GenericError {
    pub fn new(kind: GenericErrorKind) -> Self {
        GenericError { kind }
    }
}

// Implement the Display trait for GenericError
impl fmt::Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            GenericErrorKind::Unexpected(detail) => write!(f, "An unexpected error occurred: {}", detail),
            GenericErrorKind::NotImplemented => write!(f, "This feature is not implemented"),
            GenericErrorKind::InvalidInput(detail) => write!(f, "Invalid input: {}", detail),
            // Add more matches for other error kinds if needed
        }
    }
}

// Implement the standard Error trait for GenericError
impl std::error::Error for GenericError {}

// impl From<BitsError> for BGVError {
//     fn from(error: BitsError) -> BGVError {
//         BGVError::BitsError(error)
//     }
// }

// impl From<BootstrapError> for BGVError {
//     fn from(error: BootstrapError) -> BGVError {
//         BGVError::BootstrapError(error)
//     }
// }

// impl From<BootstrappableError> for BGVError {
//     fn from(error: BootstrappableError) -> BGVError {
//         BGVError::BootstrappableError(error)
//     }
// }

// impl From<CError> for BGVError {
//     fn from(error: CError) -> BGVError {
//         BGVError::CError(error)
//     }
// }

impl From<ConstructionError> for BGVError {
    fn from(error: ConstructionError) -> BGVError {
        BGVError::ConstructionError(error)
    }
}

impl From<GenericError> for BGVError {
    fn from(error: GenericError) -> BGVError {
        BGVError::GenericError(error)
    }
}

// impl From<GensError> for BGVError {
//     fn from(error: GensError) -> BGVError {
//         BGVError::GensError(error)
//     }
// }

impl From<MError> for BGVError {
    fn from(error: MError) -> BGVError {
        // BGVError::MError(error)
        match error.kind {
            MErrorKind::OutOfRange(reason) => BGVError::ConversionError {
                from: "i64",
                to: "M",
                reason,
            },
            // Map other MErrorKind variants to appropriate BGVError
            _ => BGVError::ConversionError {
                from: "M",
                to: "Metric",
                reason: format!("{:?}", error.kind),
            },
        }
    }
}

// impl From<MvecError> for BGVError {
//     fn from(error: MvecError) -> BGVError {
//         BGVError::MvecError(error)
//     }
// }

// impl From<OrdsError> for BGVError {
//     fn from(error: OrdsError) -> BGVError {
//         BGVError::OrdsError(error)
//     }
// }

// impl From<PError> for BGVError {
//     fn from(error: PError) -> BGVError {
//         BGVError::PError(error)
//     }
// }

// impl From<RError> for BGVError {
//     fn from(error: RError) -> BGVError {
//         BGVError::RError(error)
//     }
// }
