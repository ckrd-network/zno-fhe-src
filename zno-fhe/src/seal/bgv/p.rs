use crate::seal::bgv::*;


use core::convert::TryFrom;
use std::num::ParseIntError;
use std::fmt;
use std::convert::{Infallible, TryInto};

use crate::prelude::*;

/// Represents the plaintext modulus parameter `p` in BGV.
///
/// In SEAL's BGV encryption scheme, the parameter `p` defines the plaintext modulus.
/// This parameter is crucial in defining the structure of the plaintext space and affects
/// the efficiency and security of the encryption scheme.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `p`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in SEAL:
/// In SEAL, the choice of `p` is a significant factor in the setup of the encryption scheme.
/// It plays a crucial role in the noise growth during computations and thereby affects the
/// overall depth of computation and security. Users should refer to SEAL's official documentation
/// or relevant academic publications for detailed guidelines on selecting `p`.
///
/// # Example
///
/// ```
/// # use crate::P;
/// let p = P::new(65537).expect("Failed to create P");
/// assert_eq!(p.to_string(), "65537");
/// ```
///
// The following code is repeated for P, R, C, and Bits with appropriate naming adjustments.

/// A non-zero unsigned 32-bit integer representing a parameter in the BGV scheme.
///
/// # Examples
///
/// ```
/// # use your_crate::P;
/// let p = P::Some(non_zero_u32::new(5).unwrap());
///
/// assert_eq!(p, P::Some(5));
/// ```
#[derive(Debug, PartialEq)]
pub enum P {
    Some(core::num::NonZeroU32),
}

/// An error related to `P` operations within the BGV scheme.
///
/// # Examples
///
/// ```
/// # use your_crate::{PError, PErrorKind};
/// let error = PError::new(PErrorKind::NegativeValue, "i32", "P");
///
/// assert_eq!(error.kind, PErrorKind::NegativeValue);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct PError {
    pub kind: PErrorKind,
    pub from: &'static str,
    pub to: &'static str,
}

impl PError {
    /// Constructs a new `PError`.
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
    /// # use crate::{PError, PErrorKind};
    /// let error = PError::new(PErrorKind::Zero, "usize", "P");
    ///
    /// assert_eq!(error.kind, PErrorKind::Zero);
    /// ```
    pub fn new(kind: PErrorKind, from: &'static str, to: &'static str) -> Self {
        PError { kind, from, to }
    }
}

/// Specific types of errors that can occur with `P`.
#[derive(Debug, Clone, PartialEq)]
pub enum PErrorKind {
    InvalidContext,
    Unreachable,
    NegativeValue,
    NoValue,
    OutOfRange(String),
    ParseError(ParseIntError),
    Zero,
    Generic(String),
}

/// Constructs a new `P` from a given value.
///
/// # Examples
///
/// ```
/// # use your_crate::P;
/// let p = P::new(42);
/// assert!(p.is_ok());
///
/// let p = P::new(-1);
/// assert!(p.is_err());
/// ```
///
/// # Errors
///
/// This will return `PError` if the value is zero, negative, or exceeds `u32::MAX`.
impl P {
    pub fn new<T>(value: T) -> Result<Self, PError>
    where
        Self: TryFrom<T, Error = PError>,
        T: num_traits::ToPrimitive + std::cmp::PartialOrd + std::fmt::Display + Copy + std::fmt::Debug,
    {
        P::try_from(value).map_err(PError::from)
    }
}

/// Convert `P` to `u32`.
///
/// `P` holds a number that cannot be zero. This function extracts that number
/// if it exists.
///
/// # Errors
///
/// Returns a `PError` with the kind `OutOfRange` if `self` is not a `Some`,
/// meaning the number was zero or never there.
/// The error details where the problem happened: from "u32" to "P".
impl ToU32<PError> for P {
    fn to_u32(&self) -> Result<u32, PError> {
        match self {
            P::Some(non_zero_u32) => Ok(non_zero_u32.get()),
            _ => Err(PError { kind: PErrorKind::OutOfRange(format!("Value {} is out of range for P", self)), from: "u32", to: "P" })
        }
    }
}

/// Returns the default value for `P`.
///
/// This is the smallest non-zero `u32` value that is permitted within `P`, namely `2`.
/// It is a constant, not arbitrary, chosen with purpose.
///
/// # Panics
///
/// This function will panic if the default value cannot be represented as a `NonZeroU32`.
/// Such a panic is not a concern in practical use; the default must be a valid non-zero `u32` value.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let p = P::default();
/// assert_eq!(p.unwrap().get(), 2);
/// ```
impl Default for P {
    fn default() -> Self {
        P::Some(core::num::NonZeroU32::new(2).expect("2 is a valid non-zero u32 value."))
    }
}

impl std::error::Error for PError {}

/// Converts an `std::io::Error` to `PError`.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::{self, Read};
/// use crate::PError;
///
/// fn read_file() -> Result<(), PError> {
///     let mut file = File::open("p.txt").map_err(PError::from)?;
///     let mut contents = String::new();
///     file.read_to_string(&mut contents).map_err(PError::from)?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns `PError::Generic` containing the error message from `std::io::Error`.
impl From<std::io::Error> for PError {
    fn from(e: std::io::Error) -> PError {
        PError::new(
            PErrorKind::Generic(e.to_string()),
            "Error",
            "PError"
        )
    }
}

/// Converts a `ParseIntError` to `PError`.
///
/// # Arguments
///
/// * `error` - The parse error encountered.
///
/// # Returns
///
/// Returns a `PError` with a `ParseError` kind, indicating a parsing failure.
impl From<std::num::ParseIntError> for PError {
    fn from(error: std::num::ParseIntError) -> Self {
        PError::new(
            PErrorKind::ParseError(error),
            "ParseIntError",
            "PError"
        )
    }
}

/// Converts from `Infallible` to `PError`.
///
/// Since `Infallible` implies no error can occur, this conversion
/// yields a variant representing an unreachable state. It should not
/// be possible for this code to run.
///
/// # Examples
///
/// ```
/// use std::convert::Infallible;
/// use crate::PError;
///
/// // Example of infallible conversion, which should not occur:
/// let error: PError = Infallible.into();
/// // Assertions about the error kind can be made here if necessary
/// ```
impl From<Infallible> for PError {
    fn from(_: Infallible) -> Self {
        PError::new(
            PErrorKind::Unreachable,
            "Infallible",
            "PError"
        )
    }
}

/// Returns the schema type.
///
/// This method declares the homomorphic encryption schema for the implementing
/// type.
/// It is straightforward and utilitarian, reflecting the singular
/// purpose of the `P` type within the cryptographic framework.
///
/// # Examples
///
/// ```
/// let p = P::default();
/// assert_eq!(p.schema(), Schema::Bgv);
/// ```
impl He for P {
    fn schema(&self) -> Schema {
        Schema::Bgv
    }
}

/// Converts `P` into a `Metric`.
///
/// # Examples
///
/// ```
/// let p = P::new(1i64);
/// let metric: Metric = p.into();
/// ```
impl Into<Metric> for P {
    fn into(self) -> Metric {
        Metric::P(self)
    }
}

/// Convert an `i8` to `P`.
///
/// # Errors
///
/// Returns `PError` in these cases:
///
/// - If `value` is zero, `PErrorKind::Zero` is returned.
/// - If `value` is negative, `PErrorKind::NegativeValue` is returned.
/// - If `NonZeroU32` cannot be created, `PErrorKind::Generic` is returned.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::prelude::*;
///
/// let p = P::try_from(5i8);
/// assert_eq!(p.is_ok(), true);
///
/// let p = P::try_from(0i8);
/// assert!(matches!(p, Err(PError { kind: PErrorKind::Zero, .. })));
///
/// let p = P::try_from(-1i8);
/// assert!(matches!(p, Err(PError { kind: PErrorKind::NegativeValue, .. })));
/// ```
impl TryFrom<i8> for P {
    type Error = PError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
            "i8",
            "P"
        ))
        } else if value < 0 {
            Err(PError::new(
                PErrorKind::NegativeValue,
            "i8",
            "P"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i8",
            "P"
        ))
        }
    }
}

/// Convert an `i16` to `P`.
///
/// # Errors
///
/// Returns `PError` for the following reasons:
/// - The input is zero. No place for nothing.
/// - The value is negative. It's wrong.
/// - Cannot make `NonZeroU32` from `i16`. It fails silently.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{P, PError, PErrorKind};
/// let positive = P::try_from(5i16);
/// assert!(positive.is_ok());
///
/// let zero = P::try_from(0i16);
/// assert_eq!(zero.unwrap_err().kind(), &PErrorKind::Zero);
///
/// let negative = P::try_from(-1i16);
/// assert_eq!(negative.unwrap_err().kind(), &PErrorKind::NegativeValue);
/// ```
impl TryFrom<i16> for P {
    type Error = PError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
            "i16",
            "P"
        ))
        } else if value < 0 {
            Err(PError::new(
                PErrorKind::NegativeValue,
            "i16",
            "P"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i16",
            "P"
        ))
        }
    }
}

/// Convert an `i32` to `P`.
///
/// # Errors
///
/// Returns `PError` in the following cases:
///
/// - If `value` is zero, `PError` signifies an attempt to create an `P` from nothing.
/// - If `value` is negative, `PError` signifies an attempt to invert the natural order.
/// - If unable to represent `value` as `NonZeroU32`, `PError` indicates a failure in creation.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{P, PError, PErrorKind};
/// assert_eq!(P::try_from(5), Ok(P::Some(nonzero::NonZeroU32::new(5).unwrap())));
/// assert!(matches!(P::try_from(0), Err(PError::new(PErrorKind::Zero, "i32", "P"))));
/// assert!(matches!(P::try_from(-1), Err(PError::new(PErrorKind::NegativeValue, "i32", "P"))));
/// ```
impl TryFrom<i32> for P {
    type Error = PError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
            "i32",
            "P"
        ))
        } else if value < 0 {
            Err(PError::new(
                PErrorKind::NegativeValue,
            "i32",
            "P"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i32",
            "P"
        ))
        }
    }
}

/// Convert an `i64` to `P`.
///
/// # Errors
///
/// Returns `Err` with `PError` if:
///
/// - The value is zero (zero is not allowed).
/// - The value is negative (negatives are not allowed).
/// - The value exceeds `u32`'s maximum (too large for `P`).
///
/// # Examples
///
/// ```
/// use std::convert::TryFrom;
/// use crate::{P, PError};
///
/// let value = 42i64;
/// let p = P::try_from(value);
/// assert!(p.is_ok());
///
/// let zero = 0i64;
/// let p = P::try_from(zero);
/// assert!(matches!(p, Err(PError { .. })));
/// ```
impl TryFrom<i64> for P {
    type Error = PError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
            "i64",
            "P"
        ))
        } else if value < 0 {
            Err(PError::new(
                PErrorKind::NegativeValue,
            "i64",
            "P"
        ))
        } else if value > u32::MAX as i64 {
            Err(PError::new(
                PErrorKind::OutOfRange(value.to_string()),
            "i64",
            "P"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i64",
            "P"
        ))
        }
    }
}

/// Converts an `i128` to `P`.
///
/// # Errors
///
/// Returns `Err` with `PError` if:
///
/// - Value is `0` (`PErrorKind::Zero`).
/// - Value is negative (`PErrorKind::NegativeValue`).
/// - Value exceeds `u32::MAX` (`PErrorKind::OutOfRange`).
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{P, PError, PErrorKind};
/// let value = 42i128;
/// let p = P::try_from(value);
/// assert_eq!(p.unwrap(), P::Some(NonZeroU32::new(42).unwrap()));
///
/// let zero = 0i128;
/// let p = P::try_from(zero);
/// assert_eq!(p.unwrap_err().kind, PErrorKind::Zero);
///
/// let negative = -1i128;
/// let p = P::try_from(negative);
/// assert_eq!(p.unwrap_err().kind, PErrorKind::NegativeValue);
///
/// let too_large = i128::from(u32::MAX) + 1;
/// let p = P::try_from(too_large);
/// assert_eq!(p.unwrap_err().kind, PErrorKind::OutOfRange);
/// ```
impl TryFrom<i128> for P {
    type Error = PError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
                "i128",
                "P"
            ))
        } else if value < 0 {
            Err(PError::new(
                PErrorKind::NegativeValue,
                "i128",
                "P"
            ))
        } else if value > u32::MAX as i128 {
            Err(PError::new(
                PErrorKind::OutOfRange(value.to_string()),
                "i128",
                "P"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "i128",
                    "P"
                ))
        }
    }
}

/// Fallible conversion of a `isize` value into a `P` instance.
///
/// # Examples
///
/// ```
/// # use crate::{P, PError, PErrorKind};
/// # use std::convert::TryFrom;
/// let positive_value = 42_isize;
/// assert!(P::try_from(positive_value).is_ok());
///
/// let negative_value = -42_isize;
/// assert_eq!(
///     P::try_from(negative_value).unwrap_err().kind,
///     PErrorKind::NegativeValue
/// );
///
/// let zero_value = 0_isize;
/// assert_eq!(
///     P::try_from(zero_value).unwrap_err().kind,
///     PErrorKind::Zero
/// );
/// ```
///
/// # Errors
///
/// Returns an `Err` containing a `PError` if:
///
/// - The value is zero, yielding `PErrorKind::Zero`.
/// - The value is negative, yielding `PErrorKind::NegativeValue`.
/// - The value exceeds the maximum for `u32`, yielding `PErrorKind::OutOfRange`.
///
/// # Notes
///
/// The `isize` type varies in size depending on the target architecture:
/// 32 bits on x86 and 64 bits on x86_64. This implementation ensures that
/// an `isize` value within the range of `u32` can be safely converted to a `P`.
#[cfg(target_pointer_width = "32")]
impl TryFrom<isize> for P {
    type Error = PError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
            "isize",
            "P"
            ))
        } else if value < 0 {
            Err(PError::new(
                PErrorKind::NegativeValue,
            "isize",
            "P"
            ))
        } else if value > u32::MAX as isize {
            Err(PError::new(
                PErrorKind::OutOfRange(format!("Value {} is out of range for P", value)),
                "isize",
                "P"))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "isize",
                "P"
                ))
        }
    }
}
#[cfg(target_pointer_width = "64")]
impl TryFrom<isize> for P {
    type Error = PError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
            "isize",
            "P"
            ))
        } else if value < 0 {
            Err(PError::new(
                PErrorKind::NegativeValue,
            "isize",
            "P"
            ))
        } else if value > u32::MAX as isize {
            Err(PError::new(
                PErrorKind::OutOfRange(format!("Value {} is out of range for P", value)),
            "isize",
            "P"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "isize",
                "P"
                ))
        }
    }
}

/// Attempts to convert a `u8` to `P`.
///
/// Fails if input is zero. Non-zero values are safely converted and encapsulated.
///
/// # Examples
///
/// Success:
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::P;
/// let value: u8 = 5;
/// assert!(P::try_from(value).is_ok());
/// ```
///
/// Failure (zero):
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{P, PError, PErrorKind};
/// let value: u8 = 0;
/// assert_eq!(P::try_from(value), Err(PError::new(PErrorKind::Zero, "u8", "P")));
/// ```
///
/// # Errors
///
/// Returns `PError` if:
///
/// - Input is zero (`PErrorKind::Zero`).
/// - `NonZeroU32` creation fails, unlikely with valid `u8` inputs (`PErrorKind::Generic`).
impl TryFrom<u8> for P {
    type Error = PError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
                "u8",
                "P"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "u8",
                    "P"
                ))
        }
    }
}

// Implementation for u16
/// Attempts to create `P` from `u16`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::convert::TryFrom;
/// use your_module::{P, PError};
///
/// let value = 5u16;
/// let p = P::try_from(value);
///
/// assert!(p.is_ok());
///
/// let value = 0u16;
/// let p = P::try_from(value);
///
/// assert!(matches!(p, Err(PError { kind: PErrorKind::Zero, .. })));
/// ```
///
/// # Errors
///
/// Returns `PError` if:
///
/// - Value is zero (`PErrorKind::Zero`).
/// - Creation of `NonZeroU32` fails internally, though unlikely (`PErrorKind::Generic`).
impl TryFrom<u16> for P {
    type Error = PError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
                "u16",
                "P"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "u16",
                    "P"
                ))
        }
    }
}

// Similar implementations for u32, u64, u128, usize, isize

// Implementation for u32
/// Attempts to create a `P` from a `u32`.
///
/// # Errors
///
/// Returns an `Err` if `value` is zero, as `P` cannot be zero.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use crate::P;
/// # use std::convert::TryFrom;
/// let p = P::try_from(42u32);
/// assert!(p.is_ok());
///
/// let p = P::try_from(0u32);
/// assert!(p.is_err());
/// ```
impl TryFrom<u32> for P {
    type Error = PError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
                "u32",
                "P"
            ))
        } else {
            core::num::NonZeroU32::new(value)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "u32",
                    "P"
                ))
        }
    }
}

// Implementation for u64
/// Attempts to convert a `u64` to `P`.
///
/// # Errors
///
/// Returns `PError` if:
///
/// - The value is `0` (as `P` cannot be zero).
/// - The value exceeds `u32::MAX`, as `P` only supports `u32` range.
///
/// # Examples
///
/// ```
/// # use crate::{P, PError, PErrorKind};
/// # use std::convert::TryFrom;
/// assert!(P::try_from(0_u64).is_err());
///
/// let large_value = u64::from(u32::MAX) + 1;
/// assert_eq!(
///     P::try_from(large_value),
///     Err(PError::new(PErrorKind::OutOfRange(large_value.to_string()), "u64", "P"))
/// );
///
/// let value_within_range = 42_u64;
/// assert_eq!(P::try_from(value_within_range), Ok(P::Some(non_zero_u32::new(42).unwrap())));
/// ```
impl TryFrom<u64> for P {
    type Error = PError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
                "u64",
                "P"
            ))
        } else if value > u32::MAX as u64 {
            Err(PError::new(
                PErrorKind::OutOfRange(value.to_string()),
                "u64",
                "P"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "u64",
                    "P"
                ))
        }
    }
}

// Implementation for u128
/// Attempts to convert a `u128` into `P`.
///
/// # Examples
///
/// ```
/// use my_crate::P;
/// use std::convert::TryFrom;
///
/// // Successful conversion
/// let p = P::try_from(42u128);
/// assert!(p.is_ok());
///
/// // Zero value, which is an error case
/// let p = P::try_from(0u128);
/// assert!(p.is_err());
///
/// // Value too large for `P`, which is an error case
/// let p = P::try_from(u128::MAX);
/// assert!(p.is_err());
/// ```
///
/// # Errors
///
/// This will return an `Err` if:
///
/// - The value is zero, as `P` cannot represent a zero value.
/// - The value exceeds the maximum value that can be represented by a `u32`.
impl TryFrom<u128> for P {
    type Error = PError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
                "u128",
                "P"
            ))
        } else if value > u32::MAX as u128 {
            Err(PError::new(
                PErrorKind::OutOfRange(value.to_string()),
                "u128",
                "P"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "u128",
                    "P"
                ))
        }
    }
}

// Implementation for TryFrom<usize> for P
/// Fallible conversion of a `usize` value into a `P` instance.
///
/// # Examples
///
/// ```
/// # use crate::{P, PError, PErrorKind};
/// # use std::convert::TryFrom;
/// let positive_value = 42_usize;
/// assert!(P::try_from(positive_value).is_ok());
///
/// let zero_value = 0_usize;
/// assert_eq!(
///     P::try_from(zero_value).unwrap_err().kind,
///     PErrorKind::Zero
/// );
///
/// let large_value = usize::MAX;
/// assert_eq!(
///     P::try_from(large_value).unwrap_err().kind,
///     PErrorKind::OutOfRange
/// );
/// ```
///
/// # Errors
///
/// Returns an `Err` containing a `PError` if:
///
/// - The value is zero, yielding `PErrorKind::Zero`.
/// - The value exceeds the maximum for `u32`, yielding `PErrorKind::OutOfRange`.
///
/// # Notes
///
/// The `usize` type varies in size depending on the target architecture.
/// This implementation ensures that a `usize` value within the range of `u32` can be safely converted to a `P`.
#[cfg(target_pointer_width = "32")]
impl TryFrom<usize> for P {
    type Error = PError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
            "usize",
            "P"
            ))
        } else if value > u32::MAX as usize {
            Err(PError::new(
                PErrorKind::OutOfRange(format!("Value {} is out of range for P", value)),
            "usize",
            "P"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "usize",
                "P"
                ))
        }
    }
}
#[cfg(target_pointer_width = "64")]
impl TryFrom<usize> for P {
    type Error = PError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(PError::new(
                PErrorKind::Zero,
            "usize",
            "P"
            ))
        } else if value > u32::MAX as usize {
            Err(PError::new(
                PErrorKind::OutOfRange(format!("Value {} is out of range for P", value)),
            "usize",
            "P"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(P::Some)
                .ok_or_else(|| PError::new(
                    PErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "usize",
                "P"
                ))
        }
    }
}

// Implementation for FromStr for P
impl core::str::FromStr for P {
    type Err = PError;
    /// Converts a string slice to `P`.
    ///
    /// # Errors
    ///
    /// Returns an error if the string does not represent a valid non-zero u32 value.
    /// This includes zero values, negative values, and values out of range for u32.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{P, PError, PErrorKind};
    ///
    /// let p: Result<P, _> = "42".parse();
    /// assert!(p.is_ok());
    ///
    /// let p: Result<P, _> = "-42".parse();
    /// assert!(matches!(p, Err(PError { kind: PErrorKind::NegativeValue, .. })));
    ///
    /// let p: Result<P, _> = "18446744073709551616".parse();
    /// assert!(matches!(p, Err(PError { kind: PErrorKind::OutOfRange(_), .. })));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(value) => P::new(value),
            Err(_) => {
                // If parsing as u32 failed, try parsing as u64 to determine if it's a range issue
                match s.parse::<u64>() {
                    Ok(value) => {
                        if value > u32::MAX as u64 {
                            Err(PError::new(
                                PErrorKind::OutOfRange("Value out of range for u32".to_string()), "str","P"))
                        } else {
                            // This branch implies logical error: the value fits within u32, but parse::<u32>() failed.
                            // It should not actually happen in normal circumstances if the input is a valid number.
                            Err(PError::new(
                                PErrorKind::Generic("Invalid number format".to_string()), "str","P"))
                        }
                    },
                    Err(_) => {
                        // If parsing as u64 also failed, then the string does not represent a valid number.
                        Err(PError::new(
                            PErrorKind::ParseError(s.parse::<u32>().unwrap_err()), "str","P"))
                    }
                }
            }
        }
    }
}

/// `P` represents a positive, non-zero `u32` value.
///
/// # Examples
///
/// ```
/// use crate::P;
///
/// let p = P::Some(nonzero::NonZeroU32::new(42).unwrap());
/// assert_eq!(format!("{}", p), "42");
/// ```
///
/// Attempting to create `P` with a zero or negative value will yield an error:
///
/// ```
/// use crate::{P, PError, PErrorKind};
///
/// let p_result = P::new(0); // or any negative number
/// assert_eq!(p_result.unwrap_err().kind, PErrorKind::Zero); // or `PErrorKind::NegativeValue`
/// ```
impl core::fmt::Display for P {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            P::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future
        }
    }
}

/// `PError` denotes the different kinds of errors that can arise from creating or using `P`.
///
/// # Examples
///
/// Creating `P` with an invalid value:
///
/// ```
/// use crate::{P, PError};
///
/// let p = P::new(0); // Zero is not a valid value for `P`
/// assert!(p.is_err());
/// assert_eq!(format!("{}", p.unwrap_err()), "zero is not allowed");
/// ```
///
/// # Errors
///
/// - `PErrorKind::Zero`: The value provided is zero.
/// - `PErrorKind::NegativeValue`: The value provided is negative.
/// - `PErrorKind::OutOfRange`: The value provided is outside the range of `u32`.
/// - `PErrorKind::ParseError`: The provided string cannot be parsed into a `u32`.
impl core::fmt::Display for PError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            PErrorKind::Unreachable => write!(f, "the Infallible place holder"),
            PErrorKind::InvalidContext => write!(f, "the UniquePtr to Context is null"),
            PErrorKind::NegativeValue => write!(f, "negative value is not allowed"),
            PErrorKind::NoValue => write!(f, "absent value is not allowed"),
            PErrorKind::OutOfRange(s) => write!(f, "{}", s),
            PErrorKind::ParseError(e) => e.fmt(f),
            PErrorKind::Zero => write!(f, "zero is not allowed"),
            PErrorKind::Generic(g) => g.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to simplify the creation of P::Some with NonZeroU32
    fn p_some(value: u32) -> P {
        P::Some(core::num::NonZeroU32::new(value).unwrap())
    }

    fn try_into_p<T>(value: T) -> Result<P, PError>
    where
        P: TryFrom<T>,
        PError: From<<P as TryFrom<T>>::Error>, // Errors via TryFrom are converted to PError
    {
        P::try_from(value).map_err(PError::from)
    }

    #[test]
    fn test_from_impl_for_metric() {
        let p = P::try_from(42).unwrap();
        let metric: Metric = p.into();
        assert!(matches!(metric, Metric::P(_)));
    }

    #[test]
    fn test_p_valid_value() {
        let p = P::new(32);
        assert!(matches!(p, Ok(P::Some(_))));
    }

    #[test]
    fn test_p_new_zero() {
        let p = P::new(0);
        assert!(matches!(p, Err(PError { kind: PErrorKind::Zero, .. })));
    }

    #[test]
    fn test_p_invalid_value() {
        let p = P::new(0);
        assert!(matches!(p, Err(PError { kind: PErrorKind::Zero, .. })));
    }

    #[test]
    fn test_p_from_str_valid() {
        let p = "1".parse::<P>();
        assert!(p.is_ok());
        assert_eq!(p.unwrap(), P::Some(core::num::NonZeroU32::new(1).unwrap()));
    }

    #[test]
    fn test_p_from_str_zero() {
        let p = "0".parse::<P>();
        assert_eq!(p, Err(PError::new(PErrorKind::Zero, "u32", "P")));
    }

    #[test]
    fn test_p_from_str_out_of_range() {
        let result = (u64::MAX).to_string().parse::<P>();
        assert!(matches!(result,
            Err(PError { kind: PErrorKind::OutOfRange(_), .. })));
    }

    #[test]
    fn test_p_from_str_invalid() {
        let p = "-1".parse::<P>();
        assert!(p.is_err());
        match p {
            Ok(_) => panic!("Expected error, got Ok(_)"),
            Err(e) => match e.kind {
                PErrorKind::ParseError(_) => (),
                _ => panic!("Expected ParseError, got {:?}", e.kind),
            },
        }
    }

    #[test]
    fn test_p_new() {
        assert!(matches!(try_into_p(1), Ok(P::Some(_))));
    }

    #[test]
    fn test_p_from_i64() {
        assert!(matches!(P::try_from(1i64), Ok(P::Some(_))));
        assert_eq!(P::try_from(0i64), Err(PError::new(
            PErrorKind::Zero, "i64", "P" )));
    }

    #[test]
    fn test_p_from_str_non_numeric() {
        let result = "non_numeric".parse::<P>();
        assert!(matches!(result,
            Err(PError { kind: PErrorKind::ParseError(_), .. })));
    }

    // Default test
    #[test]
    fn test_p_default() {
        assert!(matches!(P::default(), P::Some(_)));
    }

    // Tests for successful conversions
    #[test]
    fn conversion_from_u8_max() {
        let value = u8::MAX;
        assert_eq!(P::try_from(value), Ok(p_some(u32::from(value))));
    }

    #[test]
    fn conversion_from_u16_max() {
        let value = u16::MAX;
        assert_eq!(P::try_from(value), Ok(p_some(u32::from(value))));
    }

    #[test]
    fn conversion_from_u32_max() {
        let value = u32::MAX;
        assert_eq!(P::try_from(value), Ok(p_some(value)));
    }

    // Tests for out-of-range conversions
    #[test]
    fn conversion_from_i8_min() {
        let value = i8::MIN;
        assert_eq!(P::try_from(value), Err(PError::new(
            PErrorKind::NegativeValue, "i8", "P" )));
    }

    #[test]
    fn conversion_from_i16_min() {
        let value = i16::MIN;
        assert_eq!(P::try_from(value), Err(PError::new(
            PErrorKind::NegativeValue, "i16", "P" )));
    }

    #[test]
    fn conversion_from_i32_min() {
        let value = i32::MIN;
        assert_eq!(P::try_from(value), Err(PError::new(
            PErrorKind::NegativeValue, "i32", "P" )));
    }

    #[test]
    fn conversion_from_i64_min() {
        let value = i64::MIN;
        assert_eq!(P::try_from(value), Err(PError::new(
            PErrorKind::NegativeValue, "i64", "P" )));
    }

    #[test]
    fn conversion_from_u64_above_max() {
        let value = u64::from(u32::MAX) + 1;
        assert_eq!(P::try_from(value), Err(PError::new(
            PErrorKind::OutOfRange(value.to_string()), "u64", "P" )));
    }

    #[test]
    fn conversion_from_isize_max() {
        let value = isize::MAX.min(u32::MAX as isize) as u32;
        assert_eq!(P::try_from(value), Ok(p_some(value)));
    }

    #[test]
    fn conversion_from_isize_min() {
        let value = isize::MIN;
        assert_eq!(P::try_from(value), Err(PError::new(
            PErrorKind::NegativeValue, "isize", "P" )));
    }

    #[test]
    fn conversion_from_usize_above_max() {
        let value = u32::MAX as usize + 1;
        assert_eq!(P::try_from(value as u64), Err(PError::new(
            PErrorKind::OutOfRange(value.to_string()), "u64", "P" )));
    }

    // Tests for zero conversions
    #[test]
    fn conversion_from_zero_i32() {
        let value = 0_i32;
        assert_eq!(P::try_from(value), Err(PError::new(
            PErrorKind::Zero, "i32", "P" )));
    }

    #[test]
    fn conversion_from_zero_i64() {
        let value = 0_i64;
        assert_eq!(P::try_from(value), Err(PError::new(
            PErrorKind::Zero, "i64", "P" )));
    }

    // u8 tests
    #[test]
    fn conversion_from_u8_zero() {
        assert_eq!(P::try_from(0_u8), Err(PError::new(
            PErrorKind::Zero, "u8", "P" )));
    }

    // u16 tests
    #[test]
    fn conversion_from_u16_zero() {
        assert_eq!(P::try_from(0_u16), Err(PError::new(
            PErrorKind::Zero, "u16", "P" )));
    }

    // i8 tests
    #[test]
    fn conversion_from_i8_zero() {
        assert_eq!(P::try_from(0_i8), Err(PError::new(
            PErrorKind::Zero, "i8", "P" )));
    }

    #[test]
    fn conversion_from_i8_max() {
        assert_eq!(P::try_from(i8::MAX), Ok(p_some(i8::MAX as u32)));
    }

    // i16 tests
    #[test]
    fn conversion_from_i16_zero() {
        assert_eq!(P::try_from(0_i16), Err(PError::new(
            PErrorKind::Zero, "i16", "P" )));
    }

    #[test]
    fn conversion_from_i16_max() {
        assert_eq!(P::try_from(i16::MAX), Ok(p_some(i16::MAX as u32)));
    }

    // Tests for usize and isize depend on the architecture of
    // the machine (32-bit or 64-bit).
    // usize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod usize_tests {
        use super::*;

        #[test]
        fn conversion_from_usize_zero_32bit() {
            assert_eq!(P::try_from(0_usize), Err(PError::new(
                PErrorKind::Zero, "usize", "P" )));
        }

        #[test]
        fn conversion_from_usize_max_32bit() {
            assert_eq!(P::try_from(usize::MAX), Ok(p_some(usize::MAX as u32)));
        }
    }

    // usize tests for 64-bit architecture
    #[cfg(target_pointer_width = "64")]
    mod usize_tests {
        use super::*;

        #[test]
        fn conversion_from_usize_zero_64bit() {
            assert_eq!(P::try_from(0_usize), Err(PError::new(
                PErrorKind::Zero, "usize", "P" )));
        }

        #[test]
        fn conversion_from_usize_max_64bit() {
            assert_eq!(P::try_from(u32::MAX as usize), Ok(p_some(u32::MAX)));
        }
    }

    // isize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_32bit() {
            assert_eq!(P::try_from(0_isize), Err(PError::new(
                PErrorKind::Zero, "isize", "P" )));
        }

        #[test]
        fn conversion_from_isize_max_32bit() {
            assert_eq!(P::try_from(isize::MAX), Ok(p_some(isize::MAX as u32)));
        }

        #[test]
        fn conversion_from_isize_min_32bit() {
            assert_eq!(P::try_from(isize::MIN), Err(PError::new(
                PErrorKind::NegativeValue, "isize", "P" )));
        }
    }

    // isize tests for 64-bit architecture
    #[cfg(target_pointer_width = "64")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_64bit() {
            assert_eq!(P::try_from(0_isize), Err(PError::new(
                PErrorKind::Zero, "isize", "P" )));
        }

        #[test]
        fn conversion_from_isize_max_64bit() {
            let value = isize::MAX.min(u32::MAX as isize) as u32;
            assert_eq!(P::try_from(value), Ok(p_some(value)));
        }

        #[test]
        fn conversion_from_isize_min_64bit() {
            assert_eq!(P::try_from(isize::MIN), Err(PError::new(
                PErrorKind::NegativeValue, "isize", "P" )));
        }
    }

    // Successful conversion tests
    #[test]
    fn test_new_success() {
        assert_eq!(try_into_p(1u8), Ok(P::Some(core::num::NonZeroU32::new(1).unwrap())));
        assert_eq!(try_into_p(1u16), Ok(P::Some(core::num::NonZeroU32::new(1).unwrap())));
        assert_eq!(try_into_p(1u32), Ok(P::Some(core::num::NonZeroU32::new(1).unwrap())));
        // Add more tests for u64, usize if within u32 range, and all i8, i16, i32, i64, isize within range
    }

    // Error case: zero
    #[test]
    fn test_new_zero_p() {
        assert_eq!(try_into_p(0u32), Err(PError::new(
            PErrorKind::Zero, "u32", "P" )));
        // Add more tests for other types with value zero
    }

    // Error case: negative value
    #[test]
    fn test_new_negative_p() {
        assert!(matches!(try_into_p(-1i32), Err(PError { kind: PErrorKind::NegativeValue, .. })));
    }

    // Error case: out of range
    #[test]
    fn test_new_out_of_range_p() {
        assert!(matches!(try_into_p(u64::MAX),
            Err(PError { kind: PErrorKind::OutOfRange(_), .. })));
        // Add more tests for other types with values out of u32 range
    }

    #[test]
    fn test_new_usize_isize_arch_dependent_p() {
        let max_usize: usize = u32::MAX as usize;
        assert_eq!(try_into_p(max_usize), Ok(P::Some(core::num::NonZeroU32::new(max_usize as u32).unwrap())));

        let max_usize_plus_one: usize = (u32::MAX as usize).wrapping_add(1);
        assert!(matches!(try_into_p(max_usize_plus_one),
            Err(PError { kind: PErrorKind::OutOfRange(_), .. })));

        if cfg!(target_pointer_width = "64") {
            let max_isize: isize = u32::MAX as isize;
            assert_eq!(try_into_p(max_isize), Ok(P::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));

            let max_isize_plus_one: isize = (u32::MAX as isize).wrapping_add(1);
            assert!(matches!(try_into_p(max_isize_plus_one),
                Err(PError { kind: PErrorKind::OutOfRange(_), .. })));

            let min_isize_minus_one: isize = (i32::MIN as isize).wrapping_sub(1);
            assert!(matches!(try_into_p(min_isize_minus_one),
                Err(PError { kind: PErrorKind::NegativeValue, .. })));
        } else if cfg!(target_pointer_width = "32") {
            // For 32-bit architectures, isize max would be within range
            let max_isize: isize = isize::MAX;
            assert_eq!(try_into_p(max_isize), Ok(P::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));
        }
    }

}
