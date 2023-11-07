pub use super::*;
use mockall::automock;

pub trait Getters {
    fn get_m(&self) -> Result<M, MError>;
    // fn get_p(&self) -> Result<P, PError>;
    // add other getter methods here . . .
}
