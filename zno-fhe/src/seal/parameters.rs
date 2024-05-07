use crate::prelude::*;

use core::fmt;

#[cfg(not(any(feature = "helib", feature = "openfhe", feature = "seal")))]
compile_error!("You must enable one of the features: `default` or`seal`");

// pub trait FheParameters {
//     type C: FheContext;
//     type E: FheError;

//     fn context(self) -> Result<Self::C, Self::E>;
// }
