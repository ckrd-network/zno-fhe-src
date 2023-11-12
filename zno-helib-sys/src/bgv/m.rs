use core::convert::TryFrom;
use std::num::ParseIntError;
use std::fmt;
use std::convert::{Infallible, TryInto};

use crate::prelude::*;

/// Represents the cyclotomic order parameter `m` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `m` determines the cyclotomic order, which is crucial for defining the polynomial ring over which computations are performed.
/// The choice of `m` affects the security and efficiency of the cryptographic operations.
///
/// The BGV scheme operates on polynomials in a ring R = Z[x]/(ϕ_m(x)), where Z represents the integers, x is an indeterminate, and ϕ_m(x) is the m-th cyclotomic polynomial. The cyclotomic polynomial is defined as the minimal polynomial over the integers for the primitive m-th roots of unity.
///
/// The order $m$ is typically chosen as a product of distinct prime numbers, where each prime is of the form $k*2^n + 1$ for integers $k$ and $n$. This choice ensures that the m-th roots of unity exist in the field of complex numbers and that $ϕ_m(x)$ splits into linear factors over the field $Z_p$ for a suitable prime $p$.
///
/// The value of $m$ is a trade-off between security and performance, and it is chosen according to the specific requirements of the application and the computational resources available.
///
/// - **Security:** The security of the scheme is based on the hardness of lattice problems, and the parameter m influences the complexity of these problems. Specifically, it affects the dimension of the lattice, which is a key factor in its security.
/// - **Efficiency:** The value of m also determines the efficiency of the scheme. Larger m can provide better security but at the cost of increased computational overhead. It affects the size of the polynomials and thus the computational complexity of the scheme's operations.
/// - **Slots for Packing:** In homomorphic encryption, one often uses "packing" techniques to encode multiple plaintext values into a single ciphertext. The parameter m affects how many such values can be packed into a single ciphertext, which is crucial for the performance of homomorphic computations on vectors of data.
///
/// ## Range in this FFI Implementation:
///
/// This FFI implementation accepts a limited range of values for `m`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in HElib:
///
/// In HElib, the choice of `m` is significant as it influences the security level and the efficiency of operations. Larger values of `m` provide better security but increase computational requirements.
/// Users should refer to HElib's official documentation or relevant cryptographic literature for detailed guidelines on selecting `m`.
///
/// # Example
///
/// ```
/// # use your_crate_name::M;
/// let m = M::new(32).expect("Should have created M");
/// assert_eq!(m.to_string(), "32");
/// ```
///


/// A non-zero unsigned 32-bit integer.
///
/// # Examples
///
/// ```
/// # use crate::M;
/// let m = M::Some(non_zero_u32::new(5).unwrap());
///
/// assert_eq!(m, M::Some(5));
/// ```
#[derive(Debug, PartialEq)]
pub enum M {
    Some(core::num::NonZeroU32),
}

/// An error related to `M` operations.
///
/// This error encapsulates various kinds of issues that can arise
/// when working with `M`, such as conversion errors or invalid values.
///
/// # Examples
///
/// Creating an error due to a negative value:
///
/// ```
/// # use crate::{MError, MErrorKind};
/// let error = MError::new(MErrorKind::NegativeValue, "i32", "M");
///
/// assert_eq!(error.kind, MErrorKind::NegativeValue);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MError {
    pub kind: MErrorKind,
    pub from: &'static str,
    pub to: &'static str,
}

impl MError {
    /// Constructs a new `MError`.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of error.
    /// * `from` - The source type that failed to convert.
    /// * `to` - The target type to which conversion was attempted.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::{MError, MErrorKind};
    /// let error = MError::new(MErrorKind::Zero, "usize", "M");
    ///
    /// assert_eq!(error.kind, MErrorKind::Zero);
    /// ```
    pub fn new(kind: MErrorKind, from: &'static str, to: &'static str) -> Self {
        MError { kind, from, to }
    }
}

/// The specific kind of error that `MError` can represent.
///
/// # Variants
///
/// * `InvalidContext` - The context in which `M` is used is invalid.
/// * `Unreachable` - An unreachable state, indicating a bug.
/// * `NegativeValue` - Attempted to create `M` from a negative value.
/// * `NoValue` - A required value for `M` was not provided.
/// * `OutOfRange` - A value was out of the valid range for `M`.
/// * `ParseError` - Failed to parse a string as `M`.
/// * `Zero` - Attempted to create `M` with a value of zero.
/// * `Generic` - A generic error, use when none of the others apply.
///
/// # Examples
///
/// ```
/// # use crate::{MError, MErrorKind};
/// let error = MError::new(MErrorKind::OutOfRange("too large".into()), "u128", "M");
///
/// assert_eq!(error.kind, MErrorKind::OutOfRange("too large".into()));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum MErrorKind {
    InvalidContext,
    Unreachable,
    NegativeValue,
    NoValue,
    OutOfRange(String),
    ParseError(ParseIntError),
    Zero,
    Generic(String),
}

/// Constructs a new `M` from a given value.
///
/// This function attempts to create an `M` from `value`. It relies on
/// `TryFrom<T>` to do so. If the conversion fails, it returns an `MError`.
///
/// # Examples
///
/// ```
/// # use crate::M;
/// let m = M::new(42);
/// assert!(m.is_ok());
///
/// let m = M::new(-1);
/// assert!(m.is_err());
/// ```
///
/// # Errors
///
/// Returns an `MError` if:
///
/// - The value is zero, which is not allowed.
/// - The value is negative, which is not allowed.
/// - The value exceeds the maximum allowed size.
///
/// # Type Parameters
///
/// - `T`: The type of value to convert from. Must be able to be compared,
///   displayed, and debugged. It should also be convertible to a primitive number.
///
/// # Panics
///
/// This function will not panic.
///
/// # See Also
///
/// See `TryFrom` and `MError` for more details.
impl M {
    pub fn new<T>(value: T) -> Result<Self, MError>
    where
        Self: TryFrom<T, Error = MError>,
        T: num_traits::ToPrimitive + std::cmp::PartialOrd + std::fmt::Display + Copy + std::fmt::Debug,
    {
        M::try_from(value).map_err(MError::from)
    }
}

/// Convert `M` to `u32`.
///
/// `M` holds a number that cannot be zero. This function extracts that number
/// if it exists.
///
/// # Errors
///
/// Returns an `MError` with the kind `OutOfRange` if `self` is not a `Some`,
/// meaning the number was zero or never there.
/// The error details where the problem happened: from "u32" to "M".
impl crate::bgv::ToU32<MError> for M {
    fn to_u32(&self) -> Result<u32, MError> {
        match self {
            M::Some(non_zero_u32) => Ok(non_zero_u32.get()),
            _ => Err(MError { kind: MErrorKind::OutOfRange(format!("Value {} is out of range for M", self)), from: "u32", to: "M" })
        }
    }
}

/// Returns the default value for `M`.
///
/// This is the smallest non-zero `u32` value that is permitted within `M`, namely `4095`.
/// It is a constant, not arbitrary, chosen with purpose.
///
/// # Panics
///
/// This function will panic if the default value cannot be represented as a `NonZeroU32`.
/// Such a panic is not a concern in practical use; `4095` is a valid non-zero `u32` value.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let m = M::default();
/// assert_eq!(m.unwrap().get(), 4095);
/// ```
impl Default for M {
    fn default() -> Self {
        M::Some(core::num::NonZeroU32::new(4095).expect("4095 is a valid non-zero u32 value."))
    }
}

impl std::error::Error for MError {}

/// Converts an `std::io::Error` to `MError`.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::{self, Read};
/// use crate::MError;
///
/// fn read_file() -> Result<(), MError> {
///     let mut file = File::open("m.txt").map_err(MError::from)?;
///     let mut contents = String::new();
///     file.read_to_string(&mut contents).map_err(MError::from)?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns `MError::Generic` containing the error message from `std::io::Error`.
impl From<std::io::Error> for MError {
    fn from(e: std::io::Error) -> MError {
        MError::new(
            MErrorKind::Generic(e.to_string()),
            "Error",
            "MError"
        )
    }
}

/// Converts a `ParseIntError` to `MError`.
///
/// # Arguments
///
/// * `error` - The parse error encountered.
///
/// # Returns
///
/// Returns an `MError` with a `ParseError` kind, indicating a parsing failure.
impl From<std::num::ParseIntError> for MError {
    fn from(error: std::num::ParseIntError) -> Self {
        MError::new(
            MErrorKind::ParseError(error),
            "ParseIntError",
            "MError"
        )
    }
}

/// Converts from `Infallible` to `MError`.
///
/// Since `Infallible` implies no error can occur, this conversion
/// yields a variant representing an unreachable state. It should not
/// be possible for this code to run.
///
/// # Examples
///
/// ```
/// use std::convert::Infallible;
/// use crate::MError;
///
/// // Example of infallible conversion, which should not occur:
/// let error: MError = Infallible.into();
/// // Assertions about the error kind can be made here if necessary
/// ```
impl From<Infallible> for MError {
    fn from(_: Infallible) -> Self {
        MError::new(
            MErrorKind::Unreachable,
            "Infallible",
            "MError"
        )
    }
}

/// Returns the schema type.
///
/// This method declares the homomorphic encryption schema for the implementing
/// type.
/// It is straightforward and utilitarian, reflecting the singular
/// purpose of the `M` type within the cryptographic framework.
///
/// # Examples
///
/// ```
/// let m = M::default();
/// assert_eq!(m.schema(), Schema::Bgv);
/// ```
impl He for M {
    fn schema(&self) -> Schema {
        Schema::Bgv
    }
}

/// Converts `M` into a `Metric`.
///
/// # Examples
///
/// ```
/// let m = M::new(1i64);
/// let metric: Metric = m.into();
/// ```
impl Into<Metric> for M {
    fn into(self) -> Metric {
        Metric::M(self)
    }
}

/// Convert an `i8` to `M`.
///
/// # Errors
///
/// Returns `MError` in these cases:
///
/// - If `value` is zero, `MErrorKind::Zero` is returned.
/// - If `value` is negative, `MErrorKind::NegativeValue` is returned.
/// - If `NonZeroU32` cannot be created, `MErrorKind::Generic` is returned.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::prelude::*;
///
/// let m = M::try_from(5i8);
/// assert_eq!(m.is_ok(), true);
///
/// let m = M::try_from(0i8);
/// assert!(matches!(m, Err(MError { kind: MErrorKind::Zero, .. })));
///
/// let m = M::try_from(-1i8);
/// assert!(matches!(m, Err(MError { kind: MErrorKind::NegativeValue, .. })));
/// ```
impl TryFrom<i8> for M {
    type Error = MError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "i8",
            "M"
        ))
        } else if value < 0 {
            Err(MError::new(
                MErrorKind::NegativeValue,
            "i8",
            "M"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i8",
            "M"
        ))
        }
    }
}

/// Convert an `i16` to `M`.
///
/// # Errors
///
/// Returns `MError` for the following reasons:
/// - The input is zero. No place for nothing.
/// - The value is negative. It's wrong.
/// - Cannot make `NonZeroU32` from `i16`. It fails silently.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{M, MError, MErrorKind};
/// let positive = M::try_from(5i16);
/// assert!(positive.is_ok());
///
/// let zero = M::try_from(0i16);
/// assert_eq!(zero.unwrap_err().kind(), &MErrorKind::Zero);
///
/// let negative = M::try_from(-1i16);
/// assert_eq!(negative.unwrap_err().kind(), &MErrorKind::NegativeValue);
/// ```
impl TryFrom<i16> for M {
    type Error = MError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "i16",
            "M"
        ))
        } else if value < 0 {
            Err(MError::new(
                MErrorKind::NegativeValue,
            "i16",
            "M"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i16",
            "M"
        ))
        }
    }
}

/// Convert an `i32` to `M`.
///
/// # Errors
///
/// Returns `MError` in the following cases:
///
/// - If `value` is zero, `MError` signifies an attempt to create an `M` from nothing.
/// - If `value` is negative, `MError` signifies an attempt to invert the natural order.
/// - If unable to represent `value` as `NonZeroU32`, `MError` indicates a failure in creation.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{M, MError, MErrorKind};
/// assert_eq!(M::try_from(5), Ok(M::Some(nonzero::NonZeroU32::new(5).unwrap())));
/// assert!(matches!(M::try_from(0), Err(MError::new(MErrorKind::Zero, "i32", "M"))));
/// assert!(matches!(M::try_from(-1), Err(MError::new(MErrorKind::NegativeValue, "i32", "M"))));
/// ```
impl TryFrom<i32> for M {
    type Error = MError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "i32",
            "M"
        ))
        } else if value < 0 {
            Err(MError::new(
                MErrorKind::NegativeValue,
            "i32",
            "M"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i32",
            "M"
        ))
        }
    }
}

/// Convert an `i64` to `M`.
///
/// # Errors
///
/// Returns `Err` with `MError` if:
///
/// - The value is zero (zero is not allowed).
/// - The value is negative (negatives are not allowed).
/// - The value exceeds `u32`'s maximum (too large for `M`).
///
/// # Examples
///
/// ```
/// use std::convert::TryFrom;
/// use crate::{M, MError};
///
/// let value = 42i64;
/// let m = M::try_from(value);
/// assert!(m.is_ok());
///
/// let zero = 0i64;
/// let m = M::try_from(zero);
/// assert!(matches!(m, Err(MError { .. })));
/// ```
impl TryFrom<i64> for M {
    type Error = MError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "i64",
            "M"
        ))
        } else if value < 0 {
            Err(MError::new(
                MErrorKind::NegativeValue,
            "i64",
            "M"
        ))
        } else if value > u32::MAX as i64 {
            Err(MError::new(
                MErrorKind::OutOfRange(value.to_string()),
            "i64",
            "M"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i64",
            "M"
        ))
        }
    }
}

/// Converts an `i128` to `M`.
///
/// # Errors
///
/// Returns an `Err` with `MError` in the following cases:
///
/// - If the value is `0`, the error kind will be `Zero`.
/// - If the value is negative, the error kind will be `NegativeValue`.
/// - If the value exceeds `u32::MAX`, the error kind will be `OutOfRange`.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{M, MError, MErrorKind};
/// let value = 42i128;
/// let m = M::try_from(value);
/// assert_eq!(m.unwrap(), M::Some(NonZeroU32::new(42).unwrap()));
///
/// let zero = 0i128;
/// let m = M::try_from(zero);
/// assert_eq!(m.unwrap_err().kind, MErrorKind::Zero);
///
/// let negative = -1i128;
/// let m = M::try_from(negative);
/// assert_eq!(m.unwrap_err().kind, MErrorKind::NegativeValue);
///
/// let too_large = i128::from(u32::MAX) + 1;
/// let m = M::try_from(too_large);
/// assert_eq!(m.unwrap_err().kind, MErrorKind::OutOfRange);
/// ```
impl TryFrom<i128> for M {
    type Error = MError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "i128",
            "M"
        ))
        } else if value < 0 {
            Err(MError::new(
                MErrorKind::NegativeValue,
            "i128",
            "M"
        ))
        } else if value > u32::MAX as i128 {
            Err(MError::new(
                MErrorKind::OutOfRange(value.to_string()),
            "i128",
            "M"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i128",
            "M"
        ))
        }
    }
}

//
// Implementations for fixed-size unsigned integers
// u8, u16, u32, u64, u128

/// Attempts to convert a `u8` to `M`.
///
/// This operation fails if the input is zero, as `M` is designed to
/// encapsulate only non-zero values. For non-zero inputs, the value is
/// safely converted and wrapped within `M`.
///
/// # Examples
///
/// Successful conversion:
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::M;
/// let value: u8 = 5;
/// assert!(M::try_from(value).is_ok());
/// ```
///
/// Failing conversion due to zero:
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{M, MError, MErrorKind};
/// let value: u8 = 0;
/// assert_eq!(M::try_from(value), Err(MError::new(MErrorKind::Zero, "u8", "M")));
/// ```
///
/// # Errors
///
/// Will return `MError` with kind:
///
/// - `MErrorKind::Zero` if the given value is zero.
/// - `MErrorKind::Generic` if a `NonZeroU32` could not be created, which should not occur with valid `u8` inputs.
impl TryFrom<u8> for M {
    type Error = MError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "u8",
            "M"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u8",
            "M"
        ))
        }
    }
}

/// Attempts to create `M` from `u16`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::convert::TryFrom;
/// use your_module::{M, MError};
///
/// let value = 5u16;
/// let m = M::try_from(value);
///
/// assert!(m.is_ok());
///
/// let value = 0u16;
/// let m = M::try_from(value);
///
/// assert!(matches!(m, Err(MError { kind: MErrorKind::Zero, .. })));
/// ```
///
/// # Errors
///
/// Returns `MError` if:
///
/// - Value is zero (`MErrorKind::Zero`).
/// - Creation of `NonZeroU32` fails internally, though unlikely (`MErrorKind::Generic`).
impl TryFrom<u16> for M {
    type Error = MError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "u16",
            "M"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u16",
            "M"
        ))
        }
    }
}

/// Attempts to create an `M` from a `u32`.
///
/// # Errors
///
/// Returns an `Err` if `value` is zero, as `M` cannot be zero.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use crate::M;
/// # use std::convert::TryFrom;
/// let m = M::try_from(42u32);
/// assert!(m.is_ok());
///
/// let m = M::try_from(0u32);
/// assert!(m.is_err());
/// ```
impl TryFrom<u32> for M {
    type Error = MError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "u32",
            "M"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u32",
            "M"
        ))
        }
    }
}
/// Attempts to convert a `u64` to `M`.
///
/// # Errors
///
/// Returns `MError` if:
///
/// - The value is `0` (as `M` cannot be zero).
/// - The value exceeds `u32::MAX`, as `M` only supports `u32` range.
///
/// # Examples
///
/// ```
/// # use crate::{M, MError, MErrorKind};
/// # use std::convert::TryFrom;
/// assert!(M::try_from(0_u64).is_err());
///
/// let large_value = u64::from(u32::MAX) + 1;
/// assert_eq!(
///     M::try_from(large_value),
///     Err(MError::new(MErrorKind::OutOfRange(large_value.to_string()), "u64", "M"))
/// );
///
/// let value_within_range = 42_u64;
/// assert_eq!(M::try_from(value_within_range), Ok(M::Some(non_zero_u32::new(42).unwrap())));
/// ```
impl TryFrom<u64> for M {
    type Error = MError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "u64",
            "M"
        ))
        } else if value > u32::MAX as u64 {
            Err(MError::new(
                MErrorKind::OutOfRange(value.to_string()),
            "u64",
            "M"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u64",
            "M"
        ))
        }
    }
}
/// Attempts to convert a `u128` into `M`.
///
/// # Examples
///
/// ```
/// use my_crate::M;
/// use std::convert::TryFrom;
///
/// // Successful conversion
/// let m = M::try_from(42u128);
/// assert!(m.is_ok());
///
/// // Zero value, which is an error case
/// let m = M::try_from(0u128);
/// assert!(m.is_err());
///
/// // Value too large for `M`, which is an error case
/// let m = M::try_from(u128::MAX);
/// assert!(m.is_err());
/// ```
///
/// # Errors
///
/// This will return an `Err` if:
///
/// - The value is zero, as `M` cannot represent a zero value.
/// - The value exceeds the maximum value that can be represented by a `u32`.
impl TryFrom<u128> for M {
    type Error = MError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "u128",
            "M"
        ))
        } else if value > u32::MAX as u128 {
            Err(MError::new(
                MErrorKind::OutOfRange(value.to_string()),
            "u128",
            "M"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u128",
            "M"
        ))
        }
    }
}

/// Fallible conversion of a `isize` value into an `M` instance.
///
/// # Examples
///
/// ```
/// # use crate::{M, MError, MErrorKind};
/// # use std::convert::TryFrom;
/// let positive_value = 42_isize;
/// assert!(M::try_from(positive_value).is_ok());
///
/// let negative_value = -42_isize;
/// assert_eq!(
///     M::try_from(negative_value).unwrap_err().kind,
///     MErrorKind::NegativeValue
/// );
///
/// let zero_value = 0_isize;
/// assert_eq!(
///     M::try_from(zero_value).unwrap_err().kind,
///     MErrorKind::Zero
/// );
/// ```
///
/// # Errors
///
/// Returns an `Err` containing an `MError` if:
///
/// - The value is zero, yielding `MErrorKind::Zero`.
/// - The value is negative, yielding `MErrorKind::NegativeValue`.
/// - The value exceeds the maximum for `u32`, yielding `MErrorKind::OutOfRange`.
///
/// # Notes
///
/// The `isize` type varies in size depending on the target architecture:
/// 32 bits on x86 and 64 bits on x86_64. This implementation ensures that
/// an `isize` value within the range of `u32` can be safely converted to an `M`.
#[cfg(target_pointer_width = "32")]
impl TryFrom<isize> for M {
    type Error = MError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "isize",
            "M"
            ))
        } else if value < 0 {
            Err(MError::new(
                MErrorKind::NegativeValue,
            "isize",
            "M"
            ))
        } else if value > u32::MAX as isize {
            Err(MError::new(
                MErrorKind::OutOfRange(format!("Value {} is out of range for M", value)),
                "isize",
                "M"))
        } else {
            // It's safe to cast to u32 because we've already checked it's within range.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "isize",
                "M"
                ))
        }
    }
}
#[cfg(target_pointer_width = "64")]
impl TryFrom<isize> for M {
    type Error = MError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "isize",
            "M"
            ))
        } else if value < 0 {
            Err(MError::new(
                MErrorKind::NegativeValue,
            "isize",
            "M"
            ))
        } else if value > u32::MAX as isize {
            Err(MError::new(
                MErrorKind::OutOfRange(format!("Value {} is out of range for M", value)),
            "isize",
            "M"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "isize",
                "M"
                ))
        }
    }
}

/// Fallible conversion of a `usize` value into an `M` instance.
///
/// # Examples
///
/// ```
/// # use crate::{M, MError, MErrorKind};
/// # use std::convert::TryFrom;
/// let positive_value = 42_isize;
/// assert!(M::try_from(positive_value).is_ok());
///
/// let negative_value = -42_isize;
/// assert_eq!(
///     M::try_from(negative_value).unwrap_err().kind,
///     MErrorKind::NegativeValue
/// );
///
/// let zero_value = 0_isize;
/// assert_eq!(
///     M::try_from(zero_value).unwrap_err().kind,
///     MErrorKind::Zero
/// );
/// ```
///
/// # Errors
///
/// Returns an `Err` containing an `MError` if:
///
/// - The value is zero, yielding `MErrorKind::Zero`.
/// - The value exceeds the maximum for `u32`, yielding `MErrorKind::OutOfRange`.
///
/// # Notes
///
/// The `usize` type varies in size depending on the target architecture:
/// 32 bits on x86 and 64 bits on x86_64. This implementation ensures that
/// an `usize` value within the range of `u32` can be safely converted to an `M`.
#[cfg(target_pointer_width = "32")]
impl TryFrom<usize> for M {
    type Error = MError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "usize",
            "M"
            ))
        } else if value > u32::MAX as usize {
            Err(MError::new(
                MErrorKind::OutOfRange(format!("Value {} is out of range for M", value)),
            "usize",
            "M"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "usize",
                "M"
                ))
        }
    }
}
#[cfg(target_pointer_width = "64")]
impl TryFrom<usize> for M {
    type Error = MError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError::new(
                MErrorKind::Zero,
            "usize",
            "M"
            ))
        } else if value > u32::MAX as usize {
            Err(MError::new(
                MErrorKind::OutOfRange(format!("Value {} is out of range for M", value)),
            "usize",
            "M"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError::new(
                    MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "usize",
                "M"
                ))
        }
    }
}

impl core::str::FromStr for M {
    type Err = MError;
    /// Converts a string slice to `M`.
    ///
    /// # Errors
    ///
    /// Returns an error if the string does not represent a valid non-zero u32 value.
    /// This includes zero values, negative values, and values out of range for u32.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{M, MError, MErrorKind};
    ///
    /// let m: Result<M, _> = "42".parse();
    /// assert!(m.is_ok());
    ///
    /// let m: Result<M, _> = "-42".parse();
    /// assert!(matches!(m, Err(MError { kind: MErrorKind::NegativeValue, .. })));
    ///
    /// let m: Result<M, _> = "18446744073709551616".parse();
    /// assert!(matches!(m, Err(MError { kind: MErrorKind::OutOfRange(_), .. })));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(value) => M::new(value),
            Err(_) => {
                // If parsing as u32 failed, try parsing as u64 to determine if it's a range issue
                match s.parse::<u64>() {
                    Ok(value) => {
                        if value > u32::MAX as u64 {
                            Err(MError::new(
                                MErrorKind::OutOfRange("Value out of range for u32".to_string()), "str","M"))
                        } else {
                            // This branch implies logical error: the value fits within u32, but parse::<u32>() failed.
                            // It should not actually happen in normal circumstances if the input is a valid number.
                            Err(MError::new(
                                MErrorKind::Generic("Invalid number format".to_string()), "str","M"))
                        }
                    },
                    Err(_) => {
                        // If parsing as u64 also failed, then the string does not represent a valid number.
                        Err(MError::new(
                            MErrorKind::ParseError(s.parse::<u32>().unwrap_err()), "str","M"))
                    }
                }
            }
        }
    }
}

/// `M` represents a positive, non-zero `u32` value.
///
/// # Examples
///
/// ```
/// use crate::M;
///
/// let m = M::Some(nonzero::NonZeroU32::new(42).unwrap());
/// assert_eq!(format!("{}", m), "42");
/// ```
///
/// Attempting to create `M` with a zero or negative value will yield an error:
///
/// ```
/// use crate::{M, MError, MErrorKind};
///
/// let m_result = M::new(0); // or any negative number
/// assert_eq!(m_result.unwrap_err().kind, MErrorKind::Zero); // or `MErrorKind::NegativeValue`
/// ```
impl core::fmt::Display for M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            M::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future
        }
    }
}

/// `MError` denotes the different kinds of errors that can arise from creating or using `M`.
///
/// # Examples
///
/// Creating `M` with an invalid value:
///
/// ```
/// use crate::{M, MError};
///
/// let m = M::new(0); // Zero is not a valid value for `M`
/// assert!(m.is_err());
/// assert_eq!(format!("{}", m.unwrap_err()), "zero is not allowed");
/// ```
///
/// # Errors
///
/// - `MErrorKind::Zero`: The value provided is zero.
/// - `MErrorKind::NegativeValue`: The value provided is negative.
/// - `MErrorKind::OutOfRange`: The value provided is outside the range of `u32`.
/// - `MErrorKind::ParseError`: The provided string cannot be parsed into a `u32`.
impl core::fmt::Display for MError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            MErrorKind::Unreachable => write!(f, "the Infallible place holder"),
            MErrorKind::InvalidContext => write!(f, "the UniquePtr to Context is null"),
            MErrorKind::NegativeValue => write!(f, "negative value is not allowed"),
            MErrorKind::NoValue => write!(f, "absent value is not allowed"),
            MErrorKind::OutOfRange(s) => write!(f, "{}", s),
            MErrorKind::ParseError(e) => e.fmt(f),
            MErrorKind::Zero => write!(f, "zero is not allowed"),
            MErrorKind::Generic(g) => g.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to simplify the creation of M::Some with NonZeroU32
    fn m_some(value: u32) -> M {
        M::Some(core::num::NonZeroU32::new(value).unwrap())
    }

    fn try_into_m<T>(value: T) -> Result<M, MError>
    where
        M: TryFrom<T>,
        MError: From<<M as TryFrom<T>>::Error>, // Errors via TryFrom are converted to MError
    {
        M::try_from(value).map_err(MError::from)
    }

    #[test]
    fn test_from_impl_for_metric() {
        let m = M::try_from(42).unwrap();
        let metric: Metric = m.into();
        assert!(matches!(metric, Metric::M(_)));
    }

    #[test]
    fn test_m_valid_value() {
        let m = M::new(32);
        assert!(matches!(m, Ok(M::Some(_))));
    }

    #[test]
    fn test_m_new_zero() {
        let m = M::new(0);
        assert!(matches!(m, Err(MError { kind: MErrorKind::Zero, .. })));
    }

    #[test]
    fn test_m_invalid_value() {
        let m = M::new(0);
        assert!(matches!(m, Err(MError { kind: MErrorKind::Zero, .. })));
    }

    #[test]
    fn test_m_from_str_valid() {
        // Parsing a valid value ("1") should succeed.
        let m = "1".parse::<M>();
        assert!(m.is_ok());
        assert_eq!(m.unwrap(), M::Some(core::num::NonZeroU32::new(1).unwrap()));
    }

        #[test]
    fn test_m_from_str_zero() {
        // Trying to parse "0" into M should yield a Zero error.
        let m = "0".parse::<M>();
        assert_eq!(m, Err(MError::new(MErrorKind::Zero, "u32", "M" )));
    }

    #[test]
    fn test_m_from_str_out_of_range() {
        let result = (u64::MAX).to_string().parse::<M>();
        assert!(matches!(result,
            Err(MError { kind: MErrorKind::OutOfRange(_), .. })));
    }

    #[test]
    fn test_m_from_str_invalid() {
        let m = "-1".parse::<M>();
        assert!(m.is_err());
        match m {
            Ok(_) => panic!("Expected error, got Ok(_)"),
            Err(e) => match e.kind {
                MErrorKind::ParseError(_) => (),
                _ => panic!("Expected ParseError, got {:?}", e.kind),
            },
        }
    }

    #[test]
    fn test_m_new() {
        assert!(matches!(try_into_m(1), Ok(M::Some(_))));
    }

    #[test]
    fn test_m_from_i64() {
        assert!(matches!(M::try_from(1i64), Ok(M::Some(_))));
        assert_eq!(M::try_from(0i64), Err(MError::new(
            MErrorKind::Zero, "i64", "M" )));
    }

    #[test]
    fn test_m_from_str_non_numeric() {
        let result = "non_numeric".parse::<M>();
        assert!(matches!(result,
            Err(MError { kind: MErrorKind::ParseError(_), .. })));
    }

    #[test]
    fn test_m_default() {
        assert!(matches!(M::default(), M::Some(_)));
    }

    // Tests for successful conversions
    #[test]
    fn conversion_from_u8_max() {
        let value = u8::MAX;
        assert_eq!(M::try_from(value), Ok(m_some(u32::from(value))));
    }

    #[test]
    fn conversion_from_u16_max() {
        let value = u16::MAX;
        assert_eq!(M::try_from(value), Ok(m_some(u32::from(value))));
    }

    #[test]
    fn conversion_from_u32_max() {
        let value = u32::MAX;
        assert_eq!(M::try_from(value), Ok(m_some(value)));
    }

    // Tests for out-of-range conversions
    #[test]
    fn conversion_from_i8_min() {
        let value = i8::MIN;
        assert_eq!(M::try_from(value), Err(MError::new(
            MErrorKind::NegativeValue, "i8", "M" )));
    }

    #[test]
    fn conversion_from_i16_min() {
        let value = i16::MIN;
        assert_eq!(M::try_from(value), Err(MError::new(
            MErrorKind::NegativeValue, "i16", "M" )));
    }

    #[test]
    fn conversion_from_i32_min() {
        let value = i32::MIN;
        assert_eq!(M::try_from(value), Err(MError::new(
            MErrorKind::NegativeValue, "i32", "M" )));
    }

    #[test]
    fn conversion_from_i64_min() {
        let value = i64::MIN;
        assert_eq!(M::try_from(value), Err(MError::new(
            MErrorKind::NegativeValue, "i64", "M" )));
    }

    #[test]
    fn conversion_from_u64_above_max() {
        let value = u64::from(u32::MAX) + 1;
        assert_eq!(M::try_from(value), Err(MError::new(
            MErrorKind::OutOfRange(value.to_string()), "u64", "M" )));
    }

    #[test]
    fn conversion_from_isize_max() {
        let value = isize::MAX.min(u32::MAX as isize) as u32;
        assert_eq!(M::try_from(value), Ok(m_some(value)));
    }

    #[test]
    fn conversion_from_isize_min() {
        let value = isize::MIN;
        assert_eq!(M::try_from(value), Err(MError::new(
            MErrorKind::NegativeValue, "isize", "M" )));
    }

    #[test]
    fn conversion_from_usize_above_max() {
        let value = u32::MAX as usize + 1;
        assert_eq!(M::try_from(value as u64), Err(MError::new(
            MErrorKind::OutOfRange(value.to_string()), "u64", "M" )));
    }

    // Tests for zero conversions
    #[test]
    fn conversion_from_zero_i32() {
        let value = 0_i32;
        assert_eq!(M::try_from(value), Err(MError::new(
            MErrorKind::Zero, "i32", "M" )));
    }

    #[test]
    fn conversion_from_zero_i64() {
        let value = 0_i64;
        assert_eq!(M::try_from(value), Err(MError::new(
            MErrorKind::Zero, "i64", "M" )));
    }

    // u8 tests
    #[test]
    fn conversion_from_u8_zero() {
        assert_eq!(M::try_from(0_u8), Err(MError::new(
            MErrorKind::Zero, "u8", "M" )));
    }

    // u16 tests
    #[test]
    fn conversion_from_u16_zero() {
        assert_eq!(M::try_from(0_u16), Err(MError::new(
            MErrorKind::Zero, "u16", "M" )));
    }

    // i8 tests
    #[test]
    fn conversion_from_i8_zero() {
        assert_eq!(M::try_from(0_i8), Err(MError::new(
            MErrorKind::Zero, "i8", "M" )));
    }

    #[test]
    fn conversion_from_i8_max() {
        assert_eq!(M::try_from(i8::MAX), Ok(m_some(i8::MAX as u32)));
    }

    // i16 tests
    #[test]
    fn conversion_from_i16_zero() {
        assert_eq!(M::try_from(0_i16), Err(MError::new(
            MErrorKind::Zero, "i16", "M" )));
    }

    #[test]
    fn conversion_from_i16_max() {
        assert_eq!(M::try_from(i16::MAX), Ok(m_some(i16::MAX as u32)));
    }

    // Tests for usize and isize depend on the architecture of
    // the machine (32-bit or 64-bit).
    // usize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod usize_tests {
        use super::*;

        #[test]
        fn conversion_from_usize_zero_32bit() {
            assert_eq!(M::try_from(0_usize), Err(MError::new(
                MErrorKind::Zero, "usize", "M" )));
        }

        #[test]
        fn conversion_from_usize_max_32bit() {
            assert_eq!(M::try_from(usize::MAX), Ok(m_some(usize::MAX as u32)));
        }
    }

    // usize tests for 64-bit architecture
    #[cfg(target_pointer_width = "64")]
    mod usize_tests {
        use super::*;

        #[test]
        fn conversion_from_usize_zero_64bit() {
            assert_eq!(M::try_from(0_usize), Err(MError::new(
                MErrorKind::Zero, "usize", "M" )));
        }

        #[test]
        fn conversion_from_usize_max_64bit() {
            assert_eq!(M::try_from(u32::MAX as usize), Ok(m_some(u32::MAX)));
        }
    }

    // isize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_32bit() {
            assert_eq!(M::try_from(0_isize), Err(MError::new(
                MErrorKind::Zero, "isize", "M" )));
        }

        #[test]
        fn conversion_from_isize_max_32bit() {
            assert_eq!(M::try_from(isize::MAX), Ok(m_some(isize::MAX as u32)));
        }

        #[test]
        fn conversion_from_isize_min_32bit() {
            assert_eq!(M::try_from(isize::MIN), Err(MError::new(
                MErrorKind::NegativeValue, "isize", "M" )));
        }
    }

    // isize tests for 64-bit architecture
    #[cfg(target_pointer_width = "64")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_64bit() {
            assert_eq!(M::try_from(0_isize), Err(MError::new(
                MErrorKind::Zero, "isize", "M" )));
        }

        #[test]
        fn conversion_from_isize_max_64bit() {
            let value = isize::MAX.min(u32::MAX as isize) as u32;
            assert_eq!(M::try_from(value), Ok(m_some(value)));
        }

        #[test]
        fn conversion_from_isize_min_64bit() {
            assert_eq!(M::try_from(isize::MIN), Err(MError::new(
                MErrorKind::NegativeValue, "isize", "M" )));
        }
    }

    // Successful conversion tests
    #[test]
    fn test_new_success() {
        assert_eq!(try_into_m(1u8), Ok(M::Some(core::num::NonZeroU32::new(1).unwrap())));
        assert_eq!(try_into_m(1u16), Ok(M::Some(core::num::NonZeroU32::new(1).unwrap())));
        assert_eq!(try_into_m(1u32), Ok(M::Some(core::num::NonZeroU32::new(1).unwrap())));
        // Add more tests for u64, usize if within u32 range, and all i8, i16, i32, i64, isize within range
    }

    // Error case: zero
    #[test]
    fn test_new_zero() {
        assert_eq!(try_into_m(0u32), Err(MError::new(
            MErrorKind::Zero, "u32", "M" )));
        // Add more tests for other types with value zero
    }

    // Error case: negative value
    #[test]
    fn test_new_negative() {
        assert!(matches!(try_into_m(-1i32), Err(MError { kind: MErrorKind::NegativeValue, .. })));
    }

    // Error case: out of range
    #[test]
    fn test_new_out_of_range() {
        assert!(matches!(try_into_m(u64::MAX),
            Err(MError { kind: MErrorKind::OutOfRange(_), .. })));
        // Add more tests for other types with values out of u32 range
    }

    #[test]
    fn test_new_usize_isize_arch_dependent() {
        let max_usize: usize = u32::MAX as usize;
        assert_eq!(try_into_m(max_usize), Ok(M::Some(core::num::NonZeroU32::new(max_usize as u32).unwrap())));

        let max_usize_plus_one: usize = (u32::MAX as usize).wrapping_add(1);
        assert!(matches!(try_into_m(max_usize_plus_one),
            Err(MError { kind: MErrorKind::OutOfRange(_), .. })));

        if cfg!(target_pointer_width = "64") {
            let max_isize: isize = u32::MAX as isize;
            assert_eq!(try_into_m(max_isize), Ok(M::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));

            let max_isize_plus_one: isize = (u32::MAX as isize).wrapping_add(1);
            assert!(matches!(try_into_m(max_isize_plus_one),
                Err(MError { kind: MErrorKind::OutOfRange(_), .. })));

            let min_isize_minus_one: isize = (i32::MIN as isize).wrapping_sub(1);
            assert!(matches!(try_into_m(min_isize_minus_one),
                Err(MError { kind: MErrorKind::NegativeValue, .. })));
        } else if cfg!(target_pointer_width = "32") {
            // For 32-bit architectures, isize max would be within range
            let max_isize: isize = isize::MAX;
            assert_eq!(try_into_m(max_isize), Ok(M::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));
        }
    }
}
