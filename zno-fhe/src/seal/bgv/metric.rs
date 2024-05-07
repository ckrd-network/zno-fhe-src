use super::m::M;
// use super::bgv::p::P;
// use super::bgv::r::R;
// use super::bgv::c::C;
// use super::bgv::bits::Bits;
// use super::bgv::gens::Gens;
// use super::bgv::ords::Ords;
// use super::bgv::mvec::Mvec;
// use super::bgv::bootstrap::Bootstrap;
// use super::bgv::bootstrappable::Bootstrappable;

use crate::prelude::*;

pub enum Metric {
    // Bits(Bits),
    // Bootstrap(Bootstrap),
    // Bootstrappable(Bootstrappable),
    // C(C),
    // Gens(Gens),
    M(M),
    // Mvec(Mvec),
    // Ords(Ords),
    // P(P),
    // R(R),
}

impl Metric {
    fn new<T, M, S>(value: T) -> Self
    where
        T: Fhe<S> + Into<Metric>,
        M: FheMetric<S>,
        S: FheScheme,
    {
        value.into()
    }
}