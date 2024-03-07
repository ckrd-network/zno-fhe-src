use core::convert::TryInto;
use std::ops::DerefMut;

pub use super::*;

// use mockall::automock;

pub trait Setters {

    fn set(self, value: Metric) -> Result<Self, BGVError>
    where
        Self: Sized; // The `Into` `M` conversion doesn't fail

    fn try_set<T>(self, value: T) -> Result<Self, BGVError>
    where
        Self: Sized,
        T: TryInto<Metric, Error=BGVError>;

    // fn set_bits<T, E>(self, value: T) -> Result<Self, BGVError>
    // where
    //     Self: Sized,
    //     T: ToU32<E>,
    //     E: Into<SetError>;

    // fn set_c<T, E>(self, value: T) -> Result<Self, BGVError>
    // where
    //     Self: Sized,
    //     T: ToU32<E>,
    //     E: Into<SetError>;

    fn set_m<T, E>(self, value: T) -> Result<Self, BGVError>
    where
        Self: Sized,
        T: ToU32<E>,
        E: Into<SetError>;

    // fn set_p<T, E>(self, value: T) -> Result<Self, BGVError>
    // where
    //     Self: Sized,
    //     T: ToU32<E>,
    //     E: Into<SetError>;

    // fn set_r<T, E>(self, value: T) -> Result<Self, BGVError>
    // where
    //     Self: Sized,
    //     T: ToU32<E>,
    //     E: Into<SetError>;
    
} // trait Setters

// // Example type implementing the Setter trait
// pub struct YourType;
// impl Setter for YourType {
//     fn set<F: Field>(self, value: F::Value) -> Result<Self, F::ErrorType> {
//         // Implementation details
//     }
// }
// let m = M::new(32);
// let instance = Builder;
// instance.set::<MField>(m);
// // Refactor to implement Field for the enum bgv::M :
// instance.set::<M>(m);
// pub trait Setter {
//     type Value: ToU32<Self::ErrorType>;
//     type ErrorType: Into<SetError>;

//     fn set<F: Field>(self, value: F::Value) -> Result<Self, F::ErrorType>
//     where
//         Self: Sized;
// }

// pub trait Setter<F: Field> {
//     fn set(self, value: F::Value) -> Result<Self, F::ErrorType>
//         where Self: Sized;
// }

// // Enum to identify which field we're setting
// pub enum FieldType {
//     M,
//     // P,
//     // other fields...
// }

// // A generalized trait to convert types into u32
// pub trait ToU32<E> {
//     fn to_u32(&self) -> Result<u32, E>;
// }

// A trait to represent field
// pub trait Field {
//     type Value: ToU32<Self::ErrorType>;
//     type ErrorType: From<Self::ErrorType>;

//     fn field_type() -> FieldType;
//     // ... rest of the trait definition
// }

// impl Field for M {
//     type Value = M; // Possibly specific to M
//     type ErrorType = MError;
//     fn field_type() -> FieldType {
//         FieldType::M
//     }
// }

// impl Field for P {
//     type Value = ToU32<Self::ErrorType>;  // Possibly specific to P
//     type ErrorType = PError;
//     fn field_type() -> FieldType {
//         FieldType::P
//     }
// }

// Shared error type
pub enum SetError {
    M(MError),
    P(PError),
    Bits(BitsError),
    Bootstrap(BootstrapError),
    Bootstrappable(BootstrappableError),
    C(CError),
    Gens(GensError),
    Mvec(MvecError),
    Ords(OrdsError),
    R(RError),
}

impl From<MError> for SetError {
    fn from(err: MError) -> SetError {
        SetError::M(err)
    }
}

impl From<PError> for SetError {
    fn from(err: PError) -> SetError {
        SetError::P(err)
    }
}

impl From<BitsError> for SetError {
    fn from(err: BitsError) -> SetError {
        SetError::Bits(err)
    }
}

impl From<BootstrapError> for SetError {
    fn from(err: BootstrapError) -> SetError {
        SetError::Bootstrap(err)
    }
}

impl From<BootstrappableError> for SetError {
    fn from(err: BootstrappableError) -> SetError {
        SetError::Bootstrappable(err)
    }
}

impl From<CError> for SetError {
    fn from(err: CError) -> SetError {
        SetError::C(err)
    }
}

impl From<GensError> for SetError {
    fn from(err: GensError) -> SetError {
        SetError::Gens(err)
    }
}

impl From<MvecError> for SetError {
    fn from(err: MvecError) -> SetError {
        SetError::Mvec(err)
    }
}

impl From<OrdsError> for SetError {
    fn from(err: OrdsError) -> SetError {
        SetError::Ords(err)
    }
}

impl From<RError> for SetError {
    fn from(err: RError) -> SetError {
        SetError::R(err)
    }
}

// Implement the conversion from SetError to BGVError
impl From<SetError> for BGVError {
    fn from(error: SetError) -> BGVError {
        match error {
            SetError::Bits(error) => BGVError::BitsError(error),
            SetError::Bootstrap(error) => BGVError::BootstrapError(error),
            SetError::Bootstrappable(error) => BGVError::BootstrappableError(error),
            SetError::C(error) => BGVError::CError(error),
            SetError::Gens(error) => BGVError::GensError(error),
            SetError::M(error) => BGVError::MError(error),
            SetError::Mvec(error) => BGVError::MvecError(error),
            SetError::Ords(error) => BGVError::OrdsError(error),
            SetError::P(error) => BGVError::PError(error),
            SetError::R(error) => BGVError::RError(error),
        }
    }
}

// impl From<PError> for SetError {
//     fn from(err: PError) -> SetError {
//         SetError::P(err)
//     }
// }

// impl Setter for Builder {
//     fn set<F: Field>(mut self, value: F::Value) -> Result<Self, F::ErrorType>
//     where Self: Sized {
//         match F::field_type() {
//             FieldType::M => match self.inner.as_mut() {
//                                     Some(inner_mut) => {
//                                         inner_mut.deref_mut().set_m(value).map_err(F::ErrorType::from)?
//                                     },
//                                     None => {
//                                         // Handle the None case, e.g., return an error
//                                     }
//                                 },
//             // FieldType::P => self.inner.set_p(value).map_err(F::ErrorType::from)?,
//             // ... Add other fields as needed
//         }
//         Ok(self)
//     }
// }
