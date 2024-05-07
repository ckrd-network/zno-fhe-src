use std::error::Error;
use core::fmt;
use core::fmt::Display;
use core::fmt::Formatter;


#[cfg(feature = "helib")]
use crate::helib::error;

#[cfg(feature = "seal")]
use crate::seal::error;
#[cfg(feature = "seal")]
use crate::seal::bgv::MError;
#[cfg(feature = "seal")]
use crate::seal::bgv::MErrorKind;

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

/// - Rust-side Null Pointer Check: On receipt of a raw pointer from C++,
///   immediately check if it's null before converting it to a safe Rust type.
///   If it's null, return an error.
/// - C++-side Error Handling: C++ functions called from Rust should signal
///   when an error has occurred. This could be returning a null pointer,
///   or some other error signaling mechanism.
/// - Error Type: The `NullPointerError` type represents an error in the
///   case of a null pointer.
///
/// A more general FFIError type represents other kinds of errors that can occur in the FFI context.

// `NullPointerError`: An error type for null pointer exceptions
#[derive(Debug, Clone)]
pub struct NullPointerError;

impl std::error::Error for NullPointerError {}

impl Display for NullPointerError {
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        write!(f, "Received null pointer from C++ library")
    }
}

#[derive(Debug, Clone)]
pub enum FFIError {
    NullPointer(NullPointerError),
    // Other FFI-related errors can be added here
    CppException(String), // This can represent an exception thrown by C++
    // ...
}

impl Error for FFIError {}

impl Display for FFIError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FFIError::NullPointer(err) => write!(f, "Null pointer error: {}", err),
            FFIError::CppException(err) => write!(f, "C++ exception: {}", err),
            // other cases as needed
        }
    }
}

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

impl std::fmt::Display for BGVError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "BGVError occurred")
    }
}

impl std::error::Error for BGVError {}

impl crate::fhe::FheError for BGVError {}

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


#[derive(Debug, Clone)]
pub enum ConversionError {
    NegativeValue,
    NoValue,                   // The Context doesn't contain a value.
    ZeroValue,                 // The value is 0, which isn't allowed in NonZeroU32.
    OutOfRange(std::num::TryFromIntError), // The value is out of the range that can be represented by a u32.
}

impl core::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            ConversionError::NegativeValue => write!(f, "Negative value is not allowed in NonZeroU32."),
            ConversionError::NoValue => write!(f, "No value present in Context."),
            ConversionError::ZeroValue => write!(f, "Zero value is not allowed in NonZeroU32."),
            ConversionError::OutOfRange(e) => write!(f, "Value out of range for u32: {}", e),
        }
    }
}

impl std::error::Error for ConversionError {}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstructionError {
    kind: ConstructionErrorKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstructionErrorKind {
    NullPointer,
    InvalidParameter,
    Generic(String),
}

impl ConstructionError {
    /// Creates a new `ConstructionError` with the specified kind.
    pub fn new(kind: ConstructionErrorKind) -> Self {
        ConstructionError { kind }
    }
}

impl std::error::Error for ConstructionError {}

// Implement From for each error type to convert into ConstructionError
impl From<std::io::Error> for ConstructionError {
    fn from(e: std::io::Error) -> ConstructionError {
        ConstructionError {
            kind: ConstructionErrorKind::Generic(e.to_string()),
        }
    }
}

impl From<NullPointerError> for ConstructionError {
    fn from(_: NullPointerError) -> Self {
        ConstructionError {
            kind: ConstructionErrorKind::NullPointer,
        }
    }
}

impl core::fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            ConstructionErrorKind::InvalidParameter => write!(f, "invalid parameter"),
            ConstructionErrorKind::NullPointer => write!(f, "Received null pointer when constructing context"),
            ConstructionErrorKind::Generic(g) => g.fmt(f),
        }
    }
}
