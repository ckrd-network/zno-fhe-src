pub use super::*;

pub trait Setters {
    fn set_m<T, E>(self, value: T) -> Result<Self, MError>
    where
        Self: Sized,
        T: ToU32<E>,
        E: Into<MError>;
    // add other setter methods here...
}
