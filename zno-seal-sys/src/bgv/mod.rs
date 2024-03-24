// Include the modules
pub mod context;


// Re-export the types for external use as `crate::bgv::<type>`
pub use self::context::*;

pub trait ToU32<E> {
    fn to_u32(&self) -> Result<u32, E>;
}

/// A marker trait for homomorphic encryption schemes.
///
/// Homomorphic encryption allows computations on encrypted data without
/// needing to decrypt it first. Implementors of this trait indicate
/// that they provide such functionality.
///
/// # Examples
///
/// ```
/// # use your_crate::{He, Schema};
/// struct Bgv; // Your implementation for BGV schema.
///
/// impl He for Bgv {
///     fn schema(&self) -> Schema {
///         Schema::Bgv
///     }
/// }
///
/// let scheme = Bgv;
/// assert_eq!(scheme.schema(), Schema::Bgv);
/// ```
pub trait He {
    /// Returns the homomorphic encryption schema being used.
    ///
    /// This method allows users to query the underlying schema of a homomorphic
    /// encryption implementation, such as BGV, BFV, CKKS, etc.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::{He, Schema};
    /// struct Bgv;
    ///
    /// impl He for Bgv {
    ///     fn schema(&self) -> Schema {
    ///         Schema::Bgv
    ///     }
    /// }
    ///
    /// let scheme = Bgv;
    /// assert_eq!(scheme.schema(), Schema::Bgv);
    /// ```
    fn schema(&self) -> Schema;
}

/// Enumerates the various homomorphic encryption schemas available.
///
/// Each variant represents a different scheme with its own set of features
/// and trade-offs.
///
/// # Examples
///
/// ```
/// # use crate::Schema;
/// let schema = Schema::Bgv;
/// ```
pub enum Schema {
    Bgv, // The Brakerski-Gentry-Vaikuntanathan (BGV) scheme.
    Bfv, // The Brakerski/Fan-Vercauteren (BFV) scheme.
    Ckks, // The Cheon-Kim-Kim-Song (CKKS) scheme.
}

/// A trait for types that represent a homomorphic encryption scheme
/// and can be converted into a `Metric`.
///
/// Types implementing `Scheme` can be used with structures or functions
/// expecting a homomorphic encryption context, and they can also be
/// converted into a `Metric` to provide a standardized interface for
/// metrics or statistics collection.
///
/// # Examples
///
/// Assuming `Bgv` is a struct representing the BGV scheme and `Metric` is
/// an enum that can represent various aspects of schemes:
///
/// ```
/// # use crate::{He, Scheme, Schema, Metric};
/// struct Bgv;
///
/// impl He for Bgv {
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
/// impl Scheme for Bgv {}
///
/// // Now Bgv can be used wherever Scheme is required
/// ```
///
/// # Errors
///
/// Implementations should ensure that the conversion to `Metric` via
/// `Into<Metric>` does not fail. If a failure is possible, it is advised
/// to implement `TryInto` trait and provide details in the implementation's
/// documentation.
pub trait Scheme: He + Into<zno_fhe::Metric> {}

// Implement the Scheme marker trait for required types
// This enforces implementation of required traits: He, Into<Metric>
// impl Scheme for Bits {}
// impl Scheme for Bootstrap {}
// impl Scheme for Bootstrappable {}
// impl Scheme for C {}
// impl Scheme for Gens {}
impl Scheme for zno_fhe::M {}
// impl Scheme for Mvec {}
// impl Scheme for Ords {}
// impl Scheme for P {}
// impl Scheme for R {}
