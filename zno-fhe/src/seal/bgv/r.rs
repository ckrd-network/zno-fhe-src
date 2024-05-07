use crate::seal::bgv::*;

use crate::prelude::*;

use core::convert::TryFrom;
use std::num::ParseIntError;
use std::fmt;
use std::convert::{Infallible, TryInto};

/// Represents the Hensel lifting degree `r` in BGV.
///
/// In SEAL's BGV encryption scheme, the parameter `r` typically refers to the Hensel lifting degree.
/// This parameter affects the encoding of integers into polynomials and plays a crucial role in
/// the efficiency of operations and the capacity of the ciphertext.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `r`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in SEAL:
/// In SEAL, the choice of `r` often depends on the desired balance between the complexity of operations
/// and the noise growth in ciphertexts. Users should refer to SEAL's official documentation or relevant
/// publications for detailed guidelines on selecting `r`.
///
/// # Example
///
/// ```
/// # use crate::R;
/// let r = R::new(32).expect("Failed to create R");
/// assert_eq!(r.to_string(), "32");
/// ```
///
/// A non-zero unsigned 32-bit integer.
///
/// # Examples
///
/// ```
/// # use your_crate::R;
/// let r = R::Some(non_zero_u32::new(5).unwrap());
///
/// assert_eq!(r, R::Some(5));
/// ```
#[derive(Debug, PartialEq)]
pub enum R {
    Some(core::num::NonZeroU32),
}

/// An error related to `R` operations.
///
/// This error encapsulates various kinds of issues that can arise
/// when working with `R`, such as conversion errors or invalid values.
///
/// # Examples
///
/// Creating an error due to a negative value:
///
/// ```
/// # use your_crate::{RError, RErrorKind};
/// let error = RError::new(RErrorKind::NegativeValue, "i32", "R");
///
/// assert_eq!(error.kind, RErrorKind::NegativeValue);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct RError {
    pub kind: RErrorKind,
    pub from: &'static str,
    pub to: &'static str,
}

impl RError {
    /// Constructs a new `RError`.
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
    /// # use crate::{RError, RErrorKind};
    /// let error = RError::new(RErrorKind::Zero, "usize", "M");
    ///
    /// assert_eq!(error.kind, RErrorKind::Zero);
    /// ```
    pub fn new(kind: RErrorKind, from: &'static str, to: &'static str) -> Self {
        RError { kind, from, to }
    }
}

/// The specific kind of error that `RError` can represent.
///
/// # Variants
///
/// * `InvalidContext` - The context in which `R` is used is invalid.
/// * `Unreachable` - An unreachable state, indicating a bug.
/// * `NegativeValue` - Attempted to create `R` from a negative value.
/// * `NoValue` - A required value for `R` was not provided.
/// * `OutOfRange` - A value was out of the valid range for `R`.
/// * `ParseError` - Failed to parse a string as `R`.
/// * `Zero` - Attempted to create `R` with a value of zero.
/// * `Generic` - A generic error, use when none of the others apply.
///
/// # Examples
///
/// ```
/// # use your_crate::{RError, RErrorKind};
/// let error = RError::new(RErrorKind::OutOfRange("too large".into()), "u128", "R");
///
/// assert_eq!(error.kind, RErrorKind::OutOfRange("too large".into()));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum RErrorKind {
    InvalidContext,
    Unreachable,
    NegativeValue,
    NoValue,
    OutOfRange(String),
    ParseError(ParseIntError),
    Zero,
    Generic(String),
}

/// Constructs a new `R` from a given value.
///
/// # Examples
///
/// ```
/// # use your_crate::R;
/// let r = R::new(42);
/// assert!(r.is_ok());
///
/// let r = R::new(-1);
/// assert!(r.is_err());
/// ```
///
/// # Errors
///
/// Returns an `RError` if the conversion fails.
impl R {
    pub fn new<T>(value: T) -> Result<Self, RError>
    where
        Self: TryFrom<T, Error = RError>,
        T: num_traits::ToPrimitive + std::cmp::PartialOrd + std::fmt::Display + Copy + std::fmt::Debug,
    {
        R::try_from(value).map_err(RError::from)
    }
}

/// Convert `R` to `u32`.
///
/// `R` holds a non-zero number. This function extracts the number if it exists.
///
/// # Errors
///
/// Returns an `RError` with the kind `OutOfRange` if `self` is not a `Some`,
/// indicating the number was zero or never present.
/// The error specifies the conversion attempt from "R" to "u32".
impl ToU32<RError> for R {
    fn to_u32(&self) -> Result<u32, RError> {
        match self {
            R::Some(non_zero_u32) => Ok(non_zero_u32.get()),
            _ => Err(RError { kind: RErrorKind::OutOfRange(format!("Value {:?} is out of range for R", self)), from: "R", to: "u32" })
        }
    }
}

/// Returns the default value for `R`.
///
/// This is the smallest non-zero `u32` value that is permitted within `R`, typically `1`.
/// This default value is chosen with intent, based on domain requirements.
///
/// # Panics
///
/// Panics if the default value cannot be represented as `NonZeroU32`. In practice, this should never happen
/// as the default must always be a valid non-zero `u32` value.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let r = R::default();
/// assert_eq!(r.unwrap().get(), 1);
/// ```
impl Default for R {
    fn default() -> Self {
        R::Some(core::num::NonZeroU32::new(1).expect("1 is a valid non-zero u32 value."))
    }
}

impl std::error::Error for RError {}

/// Converts an `std::io::Error` to `RError`.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::{self, Read};
/// use crate::RError;
///
/// fn read_file() -> Result<(), RError> {
///     let mut file = File::open("r.txt").map_err(RError::from)?;
///     let mut contents = String::new();
///     file.read_to_string(&mut contents).map_err(RError::from)?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns `RError::Generic` containing the error message from `std::io::Error`.
impl From<std::io::Error> for RError {
    fn from(e: std::io::Error) -> RError {
        RError::new(
            RErrorKind::Generic(e.to_string()),
            "Error",
            "RError"
        )
    }
}

/// Converts a `ParseIntError` to `RError`.
///
/// # Arguments
///
/// * `error` - The parse error encountered.
///
/// # Returns
///
/// Returns an `RError` with a `ParseError` kind, indicating a parsing failure.
impl From<std::num::ParseIntError> for RError {
    fn from(error: std::num::ParseIntError) -> Self {
        RError::new(
            RErrorKind::ParseError(error),
            "ParseIntError",
            "RError"
        )
    }
}

/// Converts from `Infallible` to `RError`.
///
/// Since `Infallible` implies no error can occur, this conversion
/// yields a variant representing an unreachable state. It should not
/// be possible for this code to run.
///
/// # Examples
///
/// ```
/// use std::convert::Infallible;
/// use crate::RError;
///
/// // Example of infallible conversion, which should not occur:
/// let error: RError = Infallible.into();
/// // Assertions about the error kind can be made here if necessary
/// ```
impl From<Infallible> for RError {
    fn from(_: Infallible) -> Self {
        RError::new(
            RErrorKind::Unreachable,
            "Infallible",
            "RError"
        )
    }
}

/// Defines the homomorphic encryption schema type for `R`.
///
/// The `schema` method for `R` provides a straightforward declaration of the
/// homomorphic encryption schema used.
///
/// # Examples
///
/// ```
/// let r = R::default();
/// assert_eq!(r.schema(), Schema::Bgv);
/// ```
impl Fhe for R {
    fn schema(&self) -> Schema {
        Schema::Bgv
    }
}

/// Converts `R` into a `Metric`.
///
/// This conversion allows `R` to be easily utilized where `Metric` values are expected,
/// facilitating its use in algorithmic processes.
///
/// # Examples
///
/// ```
/// let r = R::new(1i64);
/// let metric: Metric = r.into();
/// ```
impl Into<Metric> for R {
    fn into(self) -> Metric {
        Metric::R(self)
    }
}

/// Convert an `i8` to `R`.
///
/// # Errors
///
/// Returns `RError` in these cases:
///
/// - If `value` is zero, `RErrorKind::Zero` is returned.
/// - If `value` is negative, `RErrorKind::NegativeValue` is returned.
/// - If `NonZeroU32` cannot be created, `RErrorKind::Generic` is returned.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::prelude::*;
///
/// let r = R::try_from(5i8);
/// assert_eq!(r.is_ok(), true);
///
/// let r = R::try_from(0i8);
/// assert!(matches!(r, Err(RError { kind: RErrorKind::Zero, .. })));
///
/// let r = R::try_from(-1i8);
/// assert!(matches!(r, Err(RError { kind: RErrorKind::NegativeValue, .. })));
/// ```
impl TryFrom<i8> for R {
    type Error = RError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "i8",
            "R"
        ))
        } else if value < 0 {
            Err(RError::new(
                RErrorKind::NegativeValue,
            "i8",
            "R"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i8",
            "R"
        ))
        }
    }
}

/// Convert an `i16` to `R`.
///
/// # Errors
///
/// Returns `RError` for the following reasons:
/// - The input is zero. No place for nothing.
/// - The value is negative. It's wrong.
/// - Cannot make `NonZeroU32` from `i16`. It fails silently.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{R, RError, RErrorKind};
/// let positive = R::try_from(5i16);
/// assert!(positive.is_ok());
///
/// let zero = R::try_from(0i16);
/// assert_eq!(zero.unwrap_err().kind(), &RErrorKind::Zero);
///
/// let negative = R::try_from(-1i16);
/// assert_eq!(negative.unwrap_err().kind(), &RErrorKind::NegativeValue);
/// ```
impl TryFrom<i16> for R {
    type Error = RError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "i16",
            "R"
        ))
        } else if value < 0 {
            Err(RError::new(
                RErrorKind::NegativeValue,
            "i16",
            "R"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i16",
            "R"
        ))
        }
    }
}

/// Convert an `i32` to `R`.
///
/// # Errors
///
/// Returns `RError` in the following cases:
///
/// - If `value` is zero, `RError` signifies an attempt to create an `R` from nothing.
/// - If `value` is negative, `RError` signifies an attempt to invert the natural order.
/// - If unable to represent `value` as `NonZeroU32`, `RError` indicates a failure in creation.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{R, RError, RErrorKind};
/// assert_eq!(R::try_from(5), Ok(R::Some(nonzero::NonZeroU32::new(5).unwrap())));
/// assert!(matches!(R::try_from(0), Err(RError::new(RErrorKind::Zero, "i32", "R"))));
/// assert!(matches!(R::try_from(-1), Err(RError::new(RErrorKind::NegativeValue, "i32", "R"))));
/// ```
impl TryFrom<i32> for R {
    type Error = RError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "i32",
            "R"
        ))
        } else if value < 0 {
            Err(RError::new(
                RErrorKind::NegativeValue,
            "i32",
            "R"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i32",
            "R"
        ))
        }
    }
}

/// Convert an `i64` to `R`.
///
/// # Errors
///
/// Returns `Err` with `RError` if:
///
/// - The value is zero (zero is not allowed).
/// - The value is negative (negatives are not allowed).
/// - The value exceeds `u32`'s maximum (too large for `R`).
///
/// # Examples
///
/// ```
/// use std::convert::TryFrom;
/// use crate::{R, RError};
///
/// let value = 42i64;
/// let r = R::try_from(value);
/// assert!(r.is_ok());
///
/// let zero = 0i64;
/// let r = R::try_from(zero);
/// assert!(matches!(r, Err(RError { .. })));
/// ```
impl TryFrom<i64> for R {
    type Error = RError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "i64",
            "R"
        ))
        } else if value < 0 {
            Err(RError::new(
                RErrorKind::NegativeValue,
            "i64",
            "R"
        ))
        } else if value > u32::MAX as i64 {
            Err(RError::new(
                RErrorKind::OutOfRange(value.to_string()),
            "i64",
            "R"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i64",
            "R"
        ))
        }
    }
}

/// Converts an `i128` to `R`.
///
/// # Errors
///
/// Returns `Err` with `RError` if:
///
/// - Value is `0` (`RErrorKind::Zero`).
/// - Value is negative (`RErrorKind::NegativeValue`).
/// - Value exceeds `u32::MAX` (`RErrorKind::OutOfRange`).
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{R, RError, RErrorKind};
/// let value = 42i128;
/// let r = R::try_from(value);
/// assert_eq!(r.unwrap(), R::Some(NonZeroU32::new(42).unwrap()));
///
/// let zero = 0i128;
/// let r = R::try_from(zero);
/// assert_eq!(r.unwrap_err().kind, RErrorKind::Zero);
///
/// let negative = -1i128;
/// let r = R::try_from(negative);
/// assert_eq!(r.unwrap_err().kind, RErrorKind::NegativeValue);
///
/// let too_large = i128::from(u32::MAX) + 1;
/// let r = R::try_from(too_large);
/// assert_eq!(r.unwrap_err().kind, RErrorKind::OutOfRange);
/// ```
impl TryFrom<i128> for R {
    type Error = RError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
                "i128",
                "R"
            ))
        } else if value < 0 {
            Err(RError::new(
                RErrorKind::NegativeValue,
                "i128",
                "R"
            ))
        } else if value > u32::MAX as i128 {
            Err(RError::new(
                RErrorKind::OutOfRange(value.to_string()),
                "i128",
                "R"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "i128",
                    "R"
                ))
        }
    }
}

/// Fallible conversion of a `isize` value into an `R` instance.
///
/// # Examples
///
/// ```
/// # use crate::{R, RError, RErrorKind};
/// # use std::convert::TryFrom;
/// let positive_value = 42_isize;
/// assert!(R::try_from(positive_value).is_ok());
///
/// let negative_value = -42_isize;
/// assert_eq!(
///     R::try_from(negative_value).unwrap_err().kind,
///     RErrorKind::NegativeValue
/// );
///
/// let zero_value = 0_isize;
/// assert_eq!(
///     R::try_from(zero_value).unwrap_err().kind,
///     RErrorKind::Zero
/// );
/// ```
///
/// # Errors
///
/// Returns an `Err` containing an `RError` if:
///
/// - The value is zero, yielding `RErrorKind::Zero`.
/// - The value is negative, yielding `RErrorKind::NegativeValue`.
/// - The value exceeds the maximum for `u32`, yielding `RErrorKind::OutOfRange`.
///
/// # Notes
///
/// The `isize` type varies in size depending on the target architecture:
/// 32 bits on x86 and 64 bits on x86_64. This implementation ensures that
/// an `isize` value within the range of `u32` can be safely converted to an `R`.
#[cfg(target_pointer_width = "32")]
impl TryFrom<isize> for R {
    type Error = RError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "isize",
            "R"
            ))
        } else if value < 0 {
            Err(RError::new(
                RErrorKind::NegativeValue,
            "isize",
            "R"
            ))
        } else if value > u32::MAX as isize {
            Err(RError::new(
                RErrorKind::OutOfRange(format!("Value {} is out of range for R", value)),
                "isize",
                "R"))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "isize",
                "R"
                ))
        }
    }
}
#[cfg(target_pointer_width = "64")]
impl TryFrom<isize> for R {
    type Error = RError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "isize",
            "R"
            ))
        } else if value < 0 {
            Err(RError::new(
                RErrorKind::NegativeValue,
            "isize",
            "R"
            ))
        } else if value > u32::MAX as isize {
            Err(RError::new(
                RErrorKind::OutOfRange(format!("Value {} is out of range for R", value)),
            "isize",
            "R"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "isize",
                "R"
                ))
        }
    }
}

/// Attempts to convert a `u8` to `R`.
///
/// Fails if input is zero. Non-zero values are safely converted and encapsulated.
///
/// # Examples
///
/// Success:
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::R;
/// let value: u8 = 5;
/// assert!(R::try_from(value).is_ok());
/// ```
///
/// Failure (zero):
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{R, RError, RErrorKind};
/// let value: u8 = 0;
/// assert_eq!(R::try_from(value), Err(RError::new(RErrorKind::Zero, "u8", "R")));
/// ```
///
/// # Errors
///
/// Returns `RError` if:
///
/// - Input is zero (`RErrorKind::Zero`).
/// - `NonZeroU32` creation fails, unlikely with valid `u8` inputs (`RErrorKind::Generic`).
impl TryFrom<u8> for R {
    type Error = RError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
                "u8",
                "R"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "u8",
                    "R"
                ))
        }
    }
}

/// Attempts to create `R` from `u16`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::convert::TryFrom;
/// use your_module::{R, RError};
///
/// let value = 5u16;
/// let r = R::try_from(value);
///
/// assert!(r.is_ok());
///
/// let value = 0u16;
/// let r = R::try_from(value);
///
/// assert!(matches!(r, Err(RError { kind: RErrorKind::Zero, .. })));
/// ```
///
/// # Errors
///
/// Returns `RError` if:
///
/// - Value is zero (`RErrorKind::Zero`).
/// - Creation of `NonZeroU32` fails internally, though unlikely (`RErrorKind::Generic`).
impl TryFrom<u16> for R {
    type Error = RError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "u16",
            "R"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u16",
            "R"
        ))
        }
    }
}

// The implementations for u32, u64, u128, and usize will be structurally identical to the above,
// with appropriate adaptations for each type.

// For example, the implementation for u32 would look like this:

/// Attempts to create an `R` from a `u32`.
///
/// # Errors
///
/// Returns an `Err` if `value` is zero, as `R` cannot be zero.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use crate::R;
/// # use std::convert::TryFrom;
/// let r = R::try_from(42u32);
/// assert!(r.is_ok());
///
/// let r = R::try_from(0u32);
/// assert!(r.is_err());
/// ```
impl TryFrom<u32> for R {
    type Error = RError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "u32",
            "R"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u32",
            "R"
        ))
        }
    }
}

/// Attempts to convert a `u64` to `R`.
///
/// # Errors
///
/// Returns `RError` if:
///
/// - The value is `0` (as `R` cannot be zero).
/// - The value exceeds `u32::MAX`, as `R` only supports `u32` range.
///
/// # Examples
///
/// ```
/// # use crate::{R, RError, RErrorKind};
/// # use std::convert::TryFrom;
/// assert!(R::try_from(0_u64).is_err());
///
/// let large_value = u64::from(u32::MAX) + 1;
/// assert_eq!(
///     R::try_from(large_value),
///     Err(RError::new(RErrorKind::OutOfRange(large_value.to_string()), "u64", "R"))
/// );
///
/// let value_within_range = 42_u64;
/// assert_eq!(R::try_from(value_within_range), Ok(R::Some(non_zero_u32::new(42).unwrap())));
/// ```
impl TryFrom<u64> for R {
    type Error = RError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "u64",
            "R"
        ))
        } else if value > u32::MAX as u64 {
            Err(RError::new(
                RErrorKind::OutOfRange(value.to_string()),
            "u64",
            "R"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u64",
            "R"
        ))
        }
    }
}

/// Attempts to convert a `u128` into `R`.
///
/// # Examples
///
/// ```
/// use my_crate::R;
/// use std::convert::TryFrom;
///
/// // Successful conversion
/// let r = R::try_from(42u128);
/// assert!(r.is_ok());
///
/// // Zero value, which is an error case
/// let r = R::try_from(0u128);
/// assert!(r.is_err());
///
/// // Value too large for `R`, which is an error case
/// let r = R::try_from(u128::MAX);
/// assert!(r.is_err());
/// ```
///
/// # Errors
///
/// This will return an `Err` if:
///
/// - The value is zero, as `R` cannot represent a zero value.
/// - The value exceeds the maximum value that can be represented by a `u32`.
impl TryFrom<u128> for R {
    type Error = RError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "u128",
            "R"
        ))
        } else if value > u32::MAX as u128 {
            Err(RError::new(
                RErrorKind::OutOfRange(value.to_string()),
            "u128",
            "R"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u128",
            "R"
        ))
        }
    }
}

/// Fallible conversion of a `usize` value into an `R` instance.
///
/// # Examples
///
/// ```
/// # use crate::{R, RError, RErrorKind};
/// # use std::convert::TryFrom;
/// let positive_value = 42_isize;
/// assert!(R::try_from(positive_value).is_ok());
///
/// let negative_value = -42_isize;
/// assert_eq!(
///     R::try_from(negative_value).unwrap_err().kind,
///     RErrorKind::NegativeValue
/// );
///
/// let zero_value = 0_isize;
/// assert_eq!(
///     R::try_from(zero_value).unwrap_err().kind,
///     RErrorKind::Zero
/// );
/// ```
///
/// # Errors
///
/// Returns an `Err` containing an `RError` if:
///
/// - The value is zero, yielding `RErrorKind::Zero`.
/// - The value exceeds the maximum for `u32`, yielding `RErrorKind::OutOfRange`.
///
/// # Notes
///
/// The `usize` type varies in size depending on the target architecture:
/// 32 bits on x86 and 64 bits on x86_64. This implementation ensures that
/// a `usize` value within the range of `u32` can be safely converted to an `R`.
#[cfg(target_pointer_width = "32")]
impl TryFrom<usize> for R {
    type Error = RError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "usize",
            "R"
            ))
        } else if value > u32::MAX as usize {
            Err(RError::new(
                RErrorKind::OutOfRange(format!("Value {} is out of range for R", value)),
            "usize",
            "R"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "usize",
                "R"
                ))
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl TryFrom<usize> for R {
    type Error = RError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(RError::new(
                RErrorKind::Zero,
            "usize",
            "R"
            ))
        } else if value > u32::MAX as usize {
            Err(RError::new(
                RErrorKind::OutOfRange(format!("Value {} is out of range for R", value)),
            "usize",
            "R"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(R::Some)
                .ok_or_else(|| RError::new(
                    RErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "usize",
                "R"
                ))
        }
    }
}

impl core::str::FromStr for R {
    type Err = RError;
    /// Converts a string slice to `R`.
    ///
    /// # Errors
    ///
    /// Returns an error if the string does not represent a valid non-zero u32 value.
    /// This includes zero values, negative values, and values out of range for u32.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{R, RError, RErrorKind};
    ///
    /// let r: Result<R, _> = "42".parse();
    /// assert!(r.is_ok());
    ///
    /// let r: Result<R, _> = "-42".parse();
    /// assert!(matches!(r, Err(RError { kind: RErrorKind::NegativeValue, .. })));
    ///
    /// let r: Result<R, _> = "18446744073709551616".parse();
    /// assert!(matches!(r, Err(RError { kind: RErrorKind::OutOfRange(_), .. })));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(value) => R::new(value),
            Err(_) => {
                // If parsing as u32 failed, try parsing as u64 to determine if it's a range issue
                match s.parse::<u64>() {
                    Ok(value) => {
                        if value > u32::MAX as u64 {
                            Err(RError::new(
                                RErrorKind::OutOfRange("Value out of range for u32".to_string()), "str","R"))
                        } else {
                            // This branch implies logical error: the value fits within u32, but parse::<u32>() failed.
                            // It should not actually happen in normal circumstances if the input is a valid number.
                            Err(RError::new(
                                RErrorKind::Generic("Invalid number format".to_string()), "str","R"))
                        }
                    },
                    Err(_) => {
                        // If parsing as u64 also failed, then the string does not represent a valid number.
                        Err(RError::new(
                            RErrorKind::ParseError(s.parse::<u32>().unwrap_err()), "str","R"))
                    }
                }
            }
        }
    }
}

/// `R` represents a positive, non-zero `u64` value.
///
/// # Examples
///
/// ```
/// use crate::R;
///
/// let r = R::Some(nonzero::NonZeroU64::new(12345).unwrap());
/// assert_eq!(format!("{}", r), "12345");
/// ```
///
/// Attempting to create `R` with a zero or negative value will yield an error:
///
/// ```
/// use crate::{R, RError, RErrorKind};
///
/// let r_result = R::new(0); // or any negative number
/// assert_eq!(r_result.unwrap_err().kind, RErrorKind::Zero); // or `RErrorKind::NegativeValue`
/// ```
impl core::fmt::Display for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            R::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future
        }
    }
}

/// `RError` denotes the different kinds of errors that can arise from creating or using `R`.
///
/// # Examples
///
/// Creating `R` with an invalid value:
///
/// ```
/// use crate::{R, RError};
///
/// let r = R::new(0); // Zero is not a valid value for `R`
/// assert!(r.is_err());
/// assert_eq!(format!("{}", r.unwrap_err()), "zero is not allowed");
/// ```
///
/// # Errors
///
/// - `RErrorKind::Zero`: The value provided is zero.
/// - `RErrorKind::NegativeValue`: The value provided is negative.
/// - `RErrorKind::OutOfRange`: The value provided is outside the range of `u64`.
/// - `RErrorKind::ParseError`: The provided string cannot be parsed into a `u64`.
impl core::fmt::Display for RError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            RErrorKind::Unreachable => write!(f, "the Infallible place holder"),
            RErrorKind::InvalidContext => write!(f, "the UniquePtr to Context is null"),
            RErrorKind::NegativeValue => write!(f, "negative value is not allowed"),
            RErrorKind::NoValue => write!(f, "absent value is not allowed"),
            RErrorKind::OutOfRange(s) => write!(f, "{}", s),
            RErrorKind::ParseError(e) => e.fmt(f),
            RErrorKind::Zero => write!(f, "zero is not allowed"),
            RErrorKind::Generic(g) => g.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function for R::Some with NonZeroU32
    fn r_some(value: u32) -> R {
        R::Some(core::num::NonZeroU32::new(value).unwrap())
    }

    fn try_into_r<T>(value: T) -> Result<R, RError>
    where
        R: TryFrom<T>,
        RError: From<<R as TryFrom<T>>::Error>, // Convert TryFrom errors to RError
    {
        R::try_from(value).map_err(RError::from)
    }

    #[test]
    fn test_from_impl_for_metric() {
        let r = R::try_from(42).unwrap();
        let metric: Metric = r.into();
        assert!(matches!(metric, Metric::R(_)));
    }

    #[test]
    fn test_r_valid_value() {
        let r = R::new(32);
        assert!(matches!(r, Ok(R::Some(_))));
    }

    #[test]
    fn test_r_new_zero() {
        let r = R::new(0);
        assert!(matches!(r, Err(RError { kind: RErrorKind::Zero, .. })));
    }

    #[test]
    fn test_r_invalid_value() {
        let r = R::new(0);
        assert!(matches!(r, Err(RError { kind: RErrorKind::Zero, .. })));
    }

    #[test]
    fn test_r_from_str_valid() {
        // Parsing a valid value ("1") should succeed.
        let r = "1".parse::<R>();
        assert!(r.is_ok());
        assert_eq!(r.unwrap(), R::Some(core::num::NonZeroU32::new(1).unwrap()));
    }

    #[test]
    fn test_r_from_str_zero() {
        // Trying to parse "0" into R should yield a Zero error.
        let r = "0".parse::<R>();
        assert_eq!(r, Err(RError::new(RErrorKind::Zero, "u32", "R")));
    }

    #[test]
    fn test_r_from_str_out_of_range() {
        let result = (u64::MAX).to_string().parse::<R>();
        assert!(matches!(result,
            Err(RError { kind: RErrorKind::OutOfRange(_), .. })));
    }

    #[test]
    fn test_r_from_str_invalid() {
        let r = "-1".parse::<R>();
        assert!(r.is_err());
        match r {
            Ok(_) => panic!("Expected error, got Ok(_)"),
            Err(e) => match e.kind {
                RErrorKind::ParseError(_) => (),
                _ => panic!("Expected ParseError, got {:?}", e.kind),
            },
        }
    }

    #[test]
    fn test_r_new() {
        assert!(matches!(try_into_r(1), Ok(R::Some(_))));
    }

    #[test]
    fn test_r_from_i64() {
        assert!(matches!(R::try_from(1i64), Ok(R::Some(_))));
        assert_eq!(R::try_from(0i64), Err(RError::new(
            RErrorKind::Zero, "i64", "R")));
    }

    #[test]
    fn test_r_from_str_non_numeric() {
        let result = "non_numeric".parse::<R>();
        assert!(matches!(result,
            Err(RError { kind: RErrorKind::ParseError(_), .. })));
    }

    #[test]
    fn test_r_default() {
        assert!(matches!(R::default(), R::Some(_)));
    }

    // Tests for successful conversions
    #[test]
    fn conversion_from_u8_max() {
        let value = u8::MAX;
        assert_eq!(R::try_from(value), Ok(r_some(u32::from(value))));
    }

    #[test]
    fn conversion_from_u16_max() {
        let value = u16::MAX;
        assert_eq!(R::try_from(value), Ok(r_some(u32::from(value))));
    }

    #[test]
    fn conversion_from_u32_max() {
        let value = u32::MAX;
        assert_eq!(R::try_from(value), Ok(r_some(value)));
    }

    // Tests for out-of-range conversions
    #[test]
    fn conversion_from_i8_min() {
        let value = i8::MIN;
        assert_eq!(R::try_from(value), Err(RError::new(
            RErrorKind::NegativeValue, "i8", "R" )));
    }

    #[test]
    fn conversion_from_i16_min() {
        let value = i16::MIN;
        assert_eq!(R::try_from(value), Err(RError::new(
            RErrorKind::NegativeValue, "i16", "R" )));
    }

    #[test]
    fn conversion_from_i32_min() {
        let value = i32::MIN;
        assert_eq!(R::try_from(value), Err(RError::new(
            RErrorKind::NegativeValue, "i32", "R" )));
    }

    #[test]
    fn conversion_from_i64_min() {
        let value = i64::MIN;
        assert_eq!(R::try_from(value), Err(RError::new(
            RErrorKind::NegativeValue, "i64", "R" )));
    }

    #[test]
    fn conversion_from_u64_above_max() {
        let value = u64::from(u32::MAX) + 1;
        assert_eq!(R::try_from(value), Err(RError::new(
            RErrorKind::OutOfRange(value.to_string()), "u64", "R" )));
    }

    #[test]
    fn conversion_from_isize_max() {
        let value = isize::MAX.min(u32::MAX as isize) as u32;
        assert_eq!(R::try_from(value), Ok(r_some(value)));
    }

    #[test]
    fn conversion_from_isize_min() {
        let value = isize::MIN;
        assert_eq!(R::try_from(value), Err(RError::new(
            RErrorKind::NegativeValue, "isize", "R" )));
    }

    #[test]
    fn conversion_from_usize_above_max() {
        let value = u32::MAX as usize + 1;
        assert_eq!(R::try_from(value as u64), Err(RError::new(
            RErrorKind::OutOfRange(value.to_string()), "u64", "R" )));
    }

    // Tests for zero conversions
    #[test]
    fn conversion_from_zero_i32() {
        let value = 0_i32;
        assert_eq!(R::try_from(value), Err(RError::new(
            RErrorKind::Zero, "i32", "R" )));
    }

    #[test]
    fn conversion_from_zero_i64() {
        let value = 0_i64;
        assert_eq!(R::try_from(value), Err(RError::new(
            RErrorKind::Zero, "i64", "R" )));
    }

    // u8 tests
    #[test]
    fn conversion_from_u8_zero() {
        assert_eq!(R::try_from(0_u8), Err(RError::new(
            RErrorKind::Zero, "u8", "R" )));
    }

    // u16 tests
    #[test]
    fn conversion_from_u16_zero() {
        assert_eq!(R::try_from(0_u16), Err(RError::new(
            RErrorKind::Zero, "u16", "R" )));
    }

    #[test]
    fn conversion_from_i8_zero() {
        assert_eq!(R::try_from(0_i8), Err(RError::new(
            RErrorKind::Zero, "i8", "R" )));
    }

    #[test]
    fn conversion_from_i8_max() {
        assert_eq!(R::try_from(i8::MAX), Ok(r_some(i8::MAX as u32)));
    }

    // i16 tests
    #[test]
    fn conversion_from_i16_zero() {
        assert_eq!(R::try_from(0_i16), Err(RError::new(
            RErrorKind::Zero, "i16", "R" )));
    }

    #[test]
    fn conversion_from_i16_max() {
        assert_eq!(R::try_from(i16::MAX), Ok(r_some(i16::MAX as u32)));
    }

    // Tests for usize and isize depend on the architecture of
    // the machine (32-bit or 64-bit).
    // usize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod usize_tests {
        use super::*;

        #[test]
        fn conversion_from_usize_zero_32bit() {
            assert_eq!(R::try_from(0_usize), Err(RError::new(
                RErrorKind::Zero, "usize", "R" )));
        }

        #[test]
        fn conversion_from_usize_max_32bit() {
            assert_eq!(R::try_from(usize::MAX), Ok(r_some(usize::MAX as u32)));
        }
    }

    // usize tests for 64-bit architecture
    #[cfg(target_pointer_width = "64")]
    mod usize_tests {
        use super::*;

        #[test]
        fn conversion_from_usize_zero_64bit() {
            assert_eq!(R::try_from(0_usize), Err(RError::new(
                RErrorKind::Zero, "usize", "R" )));
        }

        #[test]
        fn conversion_from_usize_max_64bit() {
            assert_eq!(R::try_from(u32::MAX as usize), Ok(r_some(u32::MAX)));
        }
    }

    // isize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_32bit() {
            assert_eq!(R::try_from(0_isize), Err(RError::new(
                RErrorKind::Zero, "isize", "R" )));
        }

        #[test]
        fn conversion_from_isize_max_32bit() {
            assert_eq!(R::try_from(isize::MAX), Ok(r_some(isize::MAX as u32)));
        }

        #[test]
        fn conversion_from_isize_min_32bit() {
            assert_eq!(R::try_from(isize::MIN), Err(RError::new(
                RErrorKind::NegativeValue, "isize", "R" )));
        }
    }

    // isize tests for 64-bit architecture
    #[cfg(target_pointer_width = "64")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_64bit() {
            assert_eq!(R::try_from(0_isize), Err(RError::new(
                RErrorKind::Zero, "isize", "R" )));
        }

        #[test]
        fn conversion_from_isize_max_64bit() {
            let value = isize::MAX.min(u32::MAX as isize) as u32;
            assert_eq!(R::try_from(value), Ok(r_some(value)));
        }

        #[test]
        fn conversion_from_isize_min_64bit() {
            assert_eq!(R::try_from(isize::MIN), Err(RError::new(
                RErrorKind::NegativeValue, "isize", "R" )));
        }
    }

    // Successful conversion tests
    #[test]
    fn test_new_success() {
        assert_eq!(try_into_r(1u8), Ok(R::Some(core::num::NonZeroU32::new(1).unwrap())));
        assert_eq!(try_into_r(1u16), Ok(R::Some(core::num::NonZeroU32::new(1).unwrap())));
        assert_eq!(try_into_r(1u32), Ok(R::Some(core::num::NonZeroU32::new(1).unwrap())));
        // Add more tests for u64, usize if within u32 range, and all i8, i16, i32, i64, isize within range
    }

    // Error case: zero
    #[test]
    fn test_new_zero() {
        assert_eq!(try_into_r(0u32), Err(RError::new(
            RErrorKind::Zero, "u32", "R" )));
        // Add more tests for other types with value zero
    }

    // Error case: negative value
    #[test]
    fn test_new_negative() {
        assert!(matches!(try_into_r(-1i32), Err(RError { kind: RErrorKind::NegativeValue, .. })));
    }

    // Error case: out of range
    #[test]
    fn test_new_out_of_range() {
        assert!(matches!(try_into_r(u64::MAX),
            Err(RError { kind: RErrorKind::OutOfRange(_), .. })));
        // Add more tests for other types with values out of u32 range
    }

    #[test]
    fn test_new_usize_isize_arch_dependent() {
        let max_usize: usize = u32::MAX as usize;
        assert_eq!(try_into_r(max_usize), Ok(R::Some(core::num::NonZeroU32::new(max_usize as u32).unwrap())));

        let max_usize_plus_one: usize = (u32::MAX as usize).wrapping_add(1);
        assert!(matches!(try_into_r(max_usize_plus_one),
            Err(RError { kind: RErrorKind::OutOfRange(_), .. })));

        if cfg!(target_pointer_width = "64") {
            let max_isize: isize = u32::MAX as isize;
            assert_eq!(try_into_r(max_isize), Ok(R::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));

            let max_isize_plus_one: isize = (u32::MAX as isize).wrapping_add(1);
            assert!(matches!(try_into_r(max_isize_plus_one),
                Err(RError { kind: RErrorKind::OutOfRange(_), .. })));

            let min_isize_minus_one: isize = (i32::MIN as isize).wrapping_sub(1);
            assert!(matches!(try_into_r(min_isize_minus_one),
                Err(RError { kind: RErrorKind::NegativeValue, .. })));
        } else if cfg!(target_pointer_width = "32") {
            // For 32-bit architectures, isize max would be within range
            let max_isize: isize = isize::MAX;
            assert_eq!(try_into_r(max_isize), Ok(R::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));
        }
    }

}
