use crate::prelude::*;

// pub enum Metric {
//     Bits(Bits),
//     Bootstrap(Bootstrap),
//     Bootstrappable(Bootstrappable),
//     C(C),
//     Gens(Gens),
//     M(M),
//     Mvec(Mvec),
//     Ords(Ords),
//     P(P),
//     R(R),
// }

// // Implement Metric creation for types implementing a HE Scheme
// impl Metric {
//     fn new<T>(value: T) -> Self
//     where
//         T: Scheme + Into<Metric>,
//     {
//         value.into()
//     }
// }