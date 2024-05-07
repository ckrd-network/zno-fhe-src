use crate::seal::builder::BGVBuilder;
use crate::seal::builder::Builder;
use crate::seal::parameters::*;
use super::parameters::*;
use crate::prelude::*;

/// `Context` is a wrapper struct that holds an instance of `ffi::Context` and is generic over `P`.
/// It implements `FheContext` for any `P` that implements `FheParameters` with `E = BGVError`.
///
/// `context` is the actual `ffi::Context` instance.
/// `_marker` is a PhantomData marker, used to make `Context` generic over `P` without holding a value of type `P`.
pub struct Context<P: FheParameters> {
    context: zno_seal_sys::bgv::ffi::Context,
    _marker: std::marker::PhantomData<P>,
}

/// Implementation of `FheContext` for `Context<P>`.
///
/// `P` is a type that implements `FheParameters` with `E = BGVError`.
/// `E` is the error type, set to `BGVError`.
// Define methods for the Rust struct seal::bgv::Context.
// Logic specific to the SEAL implementation belongs here.
impl<P> FheContext for Context<P>
where
    P: FheParameters<E = BGVError>,
{
    type P = P;
    type E = BGVError;

    // Create a new instance of the C++ object Context.
    // This is safe because we're not exposing the inner pointer directly.
    // Logic specific to the SEAL BGV implementation belongs here.
    fn new(params: Self::P) -> Result<Self, Self::E> {
        let mut params = params; // Make params mutable
        let builder = BGVBuilder::new(params)
                     .set(params.m.into())?
                    //  .set(params.p.into())?
                    //  .set(params.r.into())?
                    // optional or conditional: You can call build without these.
                    // https://users.rust-lang.org/t/builder-pattern-in-rust-self-vs-mut-self-and-method-vs-associated-function/72892/2
                    // https://dev.to/mindflavor/rust-builder-pattern-with-types-3chf
                    //  .set(params.bits.into())?
                    //  .set(params.c.into())?
                    //  .set(params.l.into())
                    //  .set(params.scale.into())
                    //  .set(params.gens.into())?
                    //  .set(params.ords.into())?
                    //  .set(params.mvec.into())?
                    // Quote:
                    //     buildModChain must be called BEFORE the context is made
                    //     botstrappable (else the "powerful" basis is not initialized correctly.
                    // .set(params.modulus_chain.into())
                    //  .set(params.bootstrap.into())?
                     ;

        // Build BGV context. Consume the instance of Builder.
        // return a UniquePtr<ffi::Context>
        let context = builder.build();
        match context {
            Ok(context) => {
                // If successful, wrap the inner ffi::Context in a UniquePtr and return.
                let inner = context.inner;  // assuming context is of type ffi::Context
                // Ok(Self { inner })
                Ok(Self)
            },
            Err(_) => {
                // If there's an error during construction, return a corresponding BGVError.
                let e = ConstructionError::new(ConstructionErrorKind::Generic("An error occurred while building the BGV Context".into()));
                Err(BGVError::ConstructionError(e))
            }
        }
    }
}

pub trait FheParameters {
    type E: std::error::Error;
    fn context<C: FheContext>(self) -> Result<C, Self::E>;
}
pub trait FheContext {
    type P: FheParameters;
    type E: FheError;

    fn new(params: Self::P) -> Result<Self, Self::E>
    where
        Self: Sized;
}
