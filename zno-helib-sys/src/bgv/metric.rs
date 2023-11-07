use crate::prelude::*;

pub enum Metric {
    Bits(Bits),
    Bootstrap(Bootstrap),
    Bootstrappable(Bootstrappable),
    C(C),
    Gens(Gens),
    M(M),
    Mvec(Mvec),
    Ords(Ords),
    P(P),
    R(R),
}

// You might need some helper functions or a factory pattern to construct Metrics
// Here's a simple factory approach for illustration purposes
// Implement Metric creation for types implementing HeScheme
impl Metric {
    // Use `where` syntax to specify the trait bounds for the generic type T
    fn new<T>(value: T) -> Self
    where
        T: Scheme + Into<Metric>,
    {
        value.into()
    }
}