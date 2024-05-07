pub use crate::schema::*;

// Homomorphic error marker trait
pub trait FheError {}

/// A trait for types that represent a homomorphic encryption scheme
/// and can be converted into a `Metric`.
///
/// Here, `Fhe<S>` is a trait that requires the implementer to also implement the `FheMetric` trait. Types implementing `Fhe<S>` can be used with structures or functions
/// expecting a homomorphic encryption context, and they can also be
/// converted into a `Metric` to provide a standardized interface for
/// a metrics collection.
///
/// # Examples
///
/// Assuming `Bgv` is a struct representing the BGV scheme and `Metric` is
/// an enum that can represent various aspects of schemes:
///
/// ```
/// # use crate::{He, Fhe, Schema, Metric};
/// struct Bgv;
///
/// impl Fhe for Bgv {
///     fn schema(&self) -> Schema {
///         Schema::Bgv
///     }
/// }
///
/// // Assume Metric has a variant for BGV
/// impl From<Bgv> for Metric {
///     fn from(_: Bgv) -> Self {
///         Metric::BgvMetrics { /* ... */ }
///     }
/// }
///
/// // Now Bgv can be used wherever Fhe is required
/// ```
///
/// # Errors
///
/// Implementations should ensure that the conversion to `Metric` via
/// `Into<Metric>` does not fail. If a failure is possible, implement
/// `TryInto` trait and provide details in the implementation
/// documentation.
///
/// This is a generic or blanket implementation of the `Fhe<S>` trait for any type `M` that implements `FheMetric`.
impl<S, M> Fhe<S> for M
where
    M: FheMetric<S>,
    S: FheScheme,
{
    fn schema(&self) -> Schema {
        match self.get_schema() {
            crate::seal::bgv::Schema::Bgv => Schema::Bgv,
            // Add other cases if seal::bgv::Schema has other variants
            _ => panic!("Unhandled schema variant"), // or return a Result and use an Err here
        }
    }

    fn get_schema(&self) -> S {
        self.get_schema()
    }
}

// Homomorphic metric marker trait
pub trait FheMetric<S> where S: FheScheme {
    fn get_schema(&self) -> S;
}

// Homomorphic encryption marker trait
pub trait Fhe<S> where S: FheScheme {
    fn schema(&self) -> Schema;
    fn get_schema(&self) -> S;
}

pub trait FheScheme {}

/// The `FheParameters` trait defines a type `E` that implements `std::error::Error` and a method `context`.
/// The `context` method is generic over `C` where `C` is any type that implements `FheContext`.
/// It takes `self` and returns a `Result<C, Self::E>`.
/// The `C::new(self)` line calls the `new` method of the `FheContext` trait, which should take a `Parameters` instance and return a `C`.
///
/// Note: We want `type C` to be any type that implements the `FheContext` trait, we could use a trait object.
/// However, trait objects require dynamic dispatch and can't be used in static contexts.
/// To keep static dispatch (which is generally more efficient), we use a generic parameter with a trait bound instead.
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
