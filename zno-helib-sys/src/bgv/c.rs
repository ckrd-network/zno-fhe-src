use core::convert::TryFrom;
use std::num::ParseIntError;
use std::fmt;
use std::convert::{Infallible, TryInto};

use crate::prelude::*;

/// Represents the parameter `c` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `c` typically refers to [specific information about what `c` represents].
/// [Explain the effect of `c` on the security and efficiency of the cryptographic operations, just like with `bits`].
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `c`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `c` often depends on a trade-off between security and performance.
/// [Describe how `c` affects this trade-off]. Users should refer to HElib's official documentation or relevant publications for detailed guidelines on selecting `c`.
///
/// # Example
///
/// ```
/// # use your_crate_name::C;  // Replace `your_crate_name` with the name of your crate
/// let c = C::new(32).expect("Failed to create C");
/// assert_eq!(c.to_string(), "32");
/// ```
///
/// A non-zero unsigned 32-bit integer.
///
/// # Examples
///
/// ```
/// # use your_crate::C;
/// let c = C::Some(non_zero_u32::new(5).unwrap());
///
/// assert_eq!(c, C::Some(5));
/// ```
#[derive(Debug, PartialEq)]
pub enum C {
    Some(core::num::NonZeroU32),
}

/// An error related to `C` operations.
///
/// This error encapsulates various kinds of issues that can arise
/// when working with `C`, such as conversion errors or invalid values.
///
/// # Examples
///
/// Creating an error due to a negative value:
///
/// ```
/// # use your_crate::{CError, CErrorKind};
/// let error = CError::new(CErrorKind::NegativeValue, "i32", "C");
///
/// assert_eq!(error.kind, CErrorKind::NegativeValue);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CError {
    pub kind: CErrorKind,
    pub from: &'static str,
    pub to: &'static str,
}

impl CError {
    /// Constructs a new `CError`.
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
    /// # use your_crate::{CError, CErrorKind};
    /// let error = CError::new(CErrorKind::Zero, "usize", "C");
    ///
    /// assert_eq!(error.kind, CErrorKind::Zero);
    /// ```
    pub fn new(kind: CErrorKind, from: &'static str, to: &'static str) -> Self {
        CError { kind, from, to }
    }
}

/// The specific kind of error that `CError` can represent.
///
/// # Variants
///
/// * `InvalidContext` - The context in which `C` is used is invalid.
/// * `Unreachable` - An unreachable state, indicating a bug.
/// * `NegativeValue` - Attempted to create `C` from a negative value.
/// * `NoValue` - A required value for `C` was not provided.
/// * `OutOfRange` - A value was out of the valid range for `C`.
/// * `ParseError` - Failed to parse a string as `C`.
/// * `Zero` - Attempted to create `C` with a value of zero.
/// * `Generic` - A generic error, use when none of the others apply.
///
/// # Examples
///
/// ```
/// # use your_crate::{CError, CErrorKind};
/// let error = CError::new(CErrorKind::OutOfRange("too large".into()), "u128", "C");
///
/// assert_eq!(error.kind, CErrorKind::OutOfRange("too large".into()));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum CErrorKind {
    InvalidContext,
    Unreachable,
    NegativeValue,
    NoValue,
    OutOfRange(String),
    ParseError(ParseIntError),
    Zero,
    Generic(String),
}

/// Constructs a new `C` from a given value.
///
/// This function attempts to create a `C` from `value`. It relies on
/// `TryFrom<T>` to do so. If the conversion fails, it returns a `CError`.
///
/// # Examples
///
/// ```
/// # use your_crate::C;
/// let c = C::new(42);
/// assert!(c.is_ok());
///
/// let c = C::new(-1);
/// assert!(c.is_err());
/// ```
///
/// # Errors
///
/// Returns a `CError` if:
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
/// See `TryFrom` and `CError` for more details.
impl C {
    pub fn new<T>(value: T) -> Result<Self, CError>
    where
        Self: TryFrom<T, Error = CError>,
        T: num_traits::ToPrimitive + std::cmp::PartialOrd + std::fmt::Display + Copy + std::fmt::Debug,
    {
        C::try_from(value).map_err(CError::from)
    }
}

/// Convert `C` to `u32`.
///
/// `C` holds a non-zero number representing a parameter of the BGV scheme.
/// This function extracts that number if present.
///
/// # Errors
///
/// Returns a `CError` with the kind `OutOfRange` if `self` is not `Some`,
/// indicating the absence of a valid value. The error details the conversion attempt
/// from "u32" to "C".
impl crate::bgv::ToU32<CError> for C {
    fn to_u32(&self) -> Result<u32, CError> {
        match self {
            C::Some(non_zero_u32) => Ok(non_zero_u32.get()),
            _ => Err(CError { kind: CErrorKind::OutOfRange(format!("Value {} is out of range for C", self)), from: "u32", to: "C" })
        }
    }
}

/// Returns the default value for `C`.
///
/// This provides the smallest non-zero `u32` value allowed for `C`, namely `4095`.
/// It is a deliberate choice, not arbitrary, reflecting its significance.
///
/// # Panics
///
/// Panics if the default value cannot be represented as a `NonZeroU32`, which should not
/// occur in practice as the default must always be a valid non-zero `u32` value.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let c = C::default();
/// assert_eq!(c.unwrap().get(), 4095);
/// ```
impl Default for C {
    fn default() -> Self {
        C::Some(core::num::NonZeroU32::new(4095).expect("4095 is a valid non-zero u32 value."))
    }
}

impl std::error::Error for CError {}

/// Converts an `std::io::Error` to `CError`.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::{self, Read};
/// use crate::CError;
///
/// fn read_file() -> Result<(), CError> {
///     let mut file = File::open("c.txt").map_err(CError::from)?;
///     let mut contents = String::new();
///     file.read_to_string(&mut contents).map_err(CError::from)?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns `CError::Generic` containing the error message from `std::io::Error`.
impl From<std::io::Error> for CError {
    fn from(e: std::io::Error) -> CError {
        CError::new(
            CErrorKind::Generic(e.to_string()),
            "Error",
            "CError"
        )
    }
}

/// Converts a `ParseIntError` to `CError`.
///
/// # Arguments
///
/// * `error` - The encountered parse error.
///
/// # Returns
///
/// Returns a `CError` with a `ParseError` kind, indicating a failure in parsing.
impl From<std::num::ParseIntError> for CError {
    fn from(error: std::num::ParseIntError) -> Self {
        CError::new(
            CErrorKind::ParseError(error),
            "ParseIntError",
            "CError"
        )
    }
}

/// Converts from `Infallible` to `CError`.
///
/// Since `Infallible` implies no error is possible, this conversion
/// results in a variant representing an unreachable state. This conversion
/// should never actually be exercised in practice.
///
/// # Examples
///
/// ```
/// use std::convert::Infallible;
/// use crate::CError;
///
/// // Example usage of infallible conversion, which should not occur:
/// let error: CError = Infallible.into();
/// // Assertions about the error kind can be made here if necessary
/// ```
impl From<Infallible> for CError {
    fn from(_: Infallible) -> Self {
        CError::new(
            CErrorKind::Unreachable,
            "Infallible",
            "CError"
        )
    }
}

/// Declares the homomorphic encryption schema for `C`.
///
/// # Examples
///
/// ```
/// let c = C::default();
/// assert_eq!(c.schema(), Schema::Bgv);
/// ```
impl He for C {
    fn schema(&self) -> Schema {
        Schema::Bgv
    }
}

/// Converts `C` into a `Metric`.
///
/// # Examples
///
/// ```
/// let c = C::new(1i64);
/// let metric: Metric = c.into();
/// ```
impl Into<Metric> for C {
    fn into(self) -> Metric {
        Metric::C(self)
    }
}

/// Convert an `i8` to `C`.
///
/// # Errors
///
/// Returns `CError` in these cases:
///
/// - If `value` is zero, `CErrorKind::Zero` is returned.
/// - If `value` is negative, `CErrorKind::NegativeValue` is returned.
/// - If `NonZeroU32` cannot be created, `CErrorKind::Generic` is returned.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::prelude::*;
///
/// let c = C::try_from(5i8);
/// assert_eq!(c.is_ok(), true);
///
/// let c = C::try_from(0i8);
/// assert!(matches!(c, Err(CError { kind: CErrorKind::Zero, .. })));
///
/// let c = C::try_from(-1i8);
/// assert!(matches!(c, Err(CError { kind: CErrorKind::NegativeValue, .. })));
/// ```
impl TryFrom<i8> for C {
    type Error = CError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "i8",
            "C"
        ))
        } else if value < 0 {
            Err(CError::new(
                CErrorKind::NegativeValue,
            "i8",
            "C"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i8",
            "C"
        ))
        }
    }
}

/// Convert an `i16` to `C`.
///
/// # Errors
///
/// Returns `CError` for the following reasons:
/// - The input is zero. No place for nothing.
/// - The value is negative. It's wrong.
/// - Cannot make `NonZeroU32` from `i16`. It fails silently.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{C, CError, CErrorKind};
/// let positive = C::try_from(5i16);
/// assert!(positive.is_ok());
///
/// let zero = C::try_from(0i16);
/// assert_eq!(zero.unwrap_err().kind(), &CErrorKind::Zero);
///
/// let negative = C::try_from(-1i16);
/// assert_eq!(negative.unwrap_err().kind(), &CErrorKind::NegativeValue);
/// ```
impl TryFrom<i16> for C {
    type Error = CError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "i16",
            "C"
        ))
        } else if value < 0 {
            Err(CError::new(
                CErrorKind::NegativeValue,
            "i16",
            "C"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i16",
            "C"
        ))
        }
    }
}
