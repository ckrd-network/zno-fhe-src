use crate::seal::parameters::Parameters;
use crate::prelude::*;

// Define methods for the Rust struct seal::Context.
// Logic specific to the SEAL implementation belongs here.
impl FheContext for Context {
    type P = Parameters;
    type E = BGVError;

    // Create a new instance of the C++ object Context.
    // This is safe because we're not exposing the inner pointer directly.
    // Logic specific to the SEAL implementation belongs here.
    fn new(params: Self::P) -> Result<Self, Self::E> {
        let mut params = params; // Make params mutable
        let cb: Builder = Builder::new()
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
        let cntxt = cb.build();
        match cntxt {
            Ok(context) => {
                // If successful, wrap the inner ffi::Context in a UniquePtr and return.
                let inner = context.inner;  // assuming context is of type ffi::Context
                Ok(Self { inner })
            },
            Err(_) => {
                // If there's an error during construction, return a corresponding BGVError.
                let e = ConstructionError::new(ConstructionErrorKind::Generic("An error occurred while building the BGV Context".into()));
                Err(BGVError::ConstructionError(e))
            }
        }
    }
}

// Implement Display for printing, debugging, etc.
impl core::fmt::Display for Context {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Context") // How this type name should appear
    }
}
