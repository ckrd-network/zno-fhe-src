pub use super::*;

pub trait Getters {
    fn get_m(&self) -> Result<M, MError>;
    // add other getter methods here...
}
