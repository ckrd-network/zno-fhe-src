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

/// Convert an `i32` to `C`.
///
/// # Errors
///
/// Returns `CError` in the following cases:
///
/// - If `value` is zero, `CError` signifies an attempt to create `C` from nothing.
/// - If `value` is negative, `CError` signifies an attempt to invert the natural order.
/// - If unable to represent `value` as `NonZeroU32`, `CError` indicates a failure in creation.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{C, CError, CErrorKind};
/// assert_eq!(C::try_from(5), Ok(C::Some(nonzero::NonZeroU32::new(5).unwrap())));
/// assert!(matches!(C::try_from(0), Err(CError::new(CErrorKind::Zero, "i32", "C"))));
/// assert!(matches!(C::try_from(-1), Err(CError::new(CErrorKind::NegativeValue, "i32", "C"))));
/// ```
impl TryFrom<i32> for C {
    type Error = CError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "i32",
            "C"
        ))
        } else if value < 0 {
            Err(CError::new(
                CErrorKind::NegativeValue,
            "i32",
            "C"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i32",
            "C"
        ))
        }
    }
}

/// Convert an `i64` to `C`.
///
/// # Errors
///
/// Returns `CError` if:
///
/// - The value is zero (zero is not allowed).
/// - The value is negative (negatives are not allowed).
/// - The value exceeds `u32`'s maximum (too large for `C`).
///
/// # Examples
///
/// ```
/// use std::convert::TryFrom;
/// use crate::{C, CError};
///
/// let value = 42i64;
/// let c = C::try_from(value);
/// assert!(c.is_ok());
///
/// let zero = 0i64;
/// let c = C::try_from(zero);
/// assert!(matches!(c, Err(CError { .. })));
/// ```
impl TryFrom<i64> for C {
    type Error = CError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "i64",
            "C"
        ))
        } else if value < 0 {
            Err(CError::new(
                CErrorKind::NegativeValue,
            "i64",
            "C"
        ))
        } else if value > u32::MAX as i64 {
            Err(CError::new(
                CErrorKind::OutOfRange(value.to_string()),
            "i64",
            "C"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i64",
            "C"
        ))
        }
    }
}

/// Converts an `i128` to `C`.
///
/// # Errors
///
/// Returns `Err` with `CError` if:
///
/// - Value is `0` (`CErrorKind::Zero`).
/// - Value is negative (`CErrorKind::NegativeValue`).
/// - Value exceeds `u32::MAX` (`CErrorKind::OutOfRange`).
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{C, CError, CErrorKind};
/// let value = 42i128;
/// let c = C::try_from(value);
/// assert_eq!(c.unwrap(), C::Some(NonZeroU32::new(42).unwrap()));
///
/// let zero = 0i128;
/// let c = C::try_from(zero);
/// assert_eq!(c.unwrap_err().kind, CErrorKind::Zero);
///
/// let negative = -1i128;
/// let c = C::try_from(negative);
/// assert_eq!(c.unwrap_err().kind, CErrorKind::NegativeValue);
///
/// let too_large = i128::from(u32::MAX) + 1;
/// let c = C::try_from(too_large);
/// assert_eq!(c.unwrap_err().kind, CErrorKind::OutOfRange);
/// ```
impl TryFrom<i128> for C {
    type Error = CError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
                "i128",
                "C"
            ))
        } else if value < 0 {
            Err(CError::new(
                CErrorKind::NegativeValue,
                "i128",
                "C"
            ))
        } else if value > u32::MAX as i128 {
            Err(CError::new(
                CErrorKind::OutOfRange(value.to_string()),
                "i128",
                "C"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "i128",
                    "C"
                ))
        }
    }
}

/// Fallible conversion of an `isize` value into a `C` instance.
///
/// # Examples
///
/// ```
/// # use crate::{C, CError, CErrorKind};
/// # use std::convert::TryFrom;
/// let positive_value = 42_isize;
/// assert!(C::try_from(positive_value).is_ok());
///
/// let negative_value = -42_isize;
/// assert_eq!(
///     C::try_from(negative_value).unwrap_err().kind,
///     CErrorKind::NegativeValue
/// );
///
/// let zero_value = 0_isize;
/// assert_eq!(
///     C::try_from(zero_value).unwrap_err().kind,
///     CErrorKind::Zero
/// );
/// ```
///
/// # Errors
///
/// Returns an `Err` containing a `CError` if:
///
/// - The value is zero, yielding `CErrorKind::Zero`.
/// - The value is negative, yielding `CErrorKind::NegativeValue`.
/// - The value exceeds the maximum for `u32`, yielding `CErrorKind::OutOfRange`.
///
/// # Notes
///
/// The `isize` type varies in size depending on the target architecture:
/// 32 bits on x86 and 64 bits on x86_64. This implementation ensures that
/// an `isize` value within the range of `u32` can be safely converted to `C`.
#[cfg(target_pointer_width = "32")]
impl TryFrom<isize> for C {
    type Error = CError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "isize",
            "C"
            ))
        } else if value < 0 {
            Err(CError::new(
                CErrorKind::NegativeValue,
            "isize",
            "C"
            ))
        } else if value > u32::MAX as isize {
            Err(CError::new(
                CErrorKind::OutOfRange(format!("Value {} is out of range for C", value)),
                "isize",
                "C"))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "isize",
                "C"
                ))
        }
    }
}
#[cfg(target_pointer_width = "64")]
impl TryFrom<isize> for C {
    type Error = CError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "isize",
            "C"
            ))
        } else if value < 0 {
            Err(CError::new(
                CErrorKind::NegativeValue,
            "isize",
            "C"
            ))
        } else if value > u32::MAX as isize {
            Err(CError::new(
                CErrorKind::OutOfRange(format!("Value {} is out of range for C", value)),
            "isize",
            "C"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "isize",
                "C"
                ))
        }
    }
}

/// Attempts to convert a `u8` to `C`.
///
/// Fails if input is zero. Non-zero values are safely converted and encapsulated.
///
/// # Examples
///
/// Success:
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::C;
/// let value: u8 = 5;
/// assert!(C::try_from(value).is_ok());
/// ```
///
/// Failure (zero):
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{C, CError, CErrorKind};
/// let value: u8 = 0;
/// assert_eq!(C::try_from(value), Err(CError::new(CErrorKind::Zero, "u8", "C")));
/// ```
///
/// # Errors
///
/// Returns `CError` if:
///
/// - Input is zero (`CErrorKind::Zero`).
/// - `NonZeroU32` creation fails, unlikely with valid `u8` inputs (`CErrorKind::Generic`).
impl TryFrom<u8> for C {
    type Error = CError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
                "u8",
                "C"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "u8",
                    "C"
                ))
        }
    }
}

/// Attempts to create `C` from `u16`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::convert::TryFrom;
/// use your_module::{C, CError};
///
/// let value = 5u16;
/// let c = C::try_from(value);
///
/// assert!(c.is_ok());
///
/// let value = 0u16;
/// let c = C::try_from(value);
///
/// assert!(matches!(c, Err(CError { kind: CErrorKind::Zero, .. })));
/// ```
///
/// # Errors
///
/// Returns `CError` if:
///
/// - Value is zero (`CErrorKind::Zero`).
/// - Creation of `NonZeroU32` fails internally, though unlikely (`CErrorKind::Generic`).
impl TryFrom<u16> for C {
    type Error = CError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "u16",
            "C"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u16",
            "C"
        ))
        }
    }
}

/// Attempts to create an `C` from a `u32`.
///
/// # Errors
///
/// Returns an `Err` if `value` is zero, as `C` cannot be zero.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use crate::C;
/// # use std::convert::TryFrom;
/// let c = C::try_from(42u32);
/// assert!(c.is_ok());
///
/// let c = C::try_from(0u32);
/// assert!(c.is_err());
/// ```
impl TryFrom<u32> for C {
    type Error = CError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "u32",
            "C"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u32",
            "C"
        ))
        }
    }
}

/// Attempts to convert a `u64` to `C`.
///
/// # Errors
///
/// Returns `CError` if:
///
/// - The value is `0` (as `C` cannot be zero).
/// - The value exceeds `u32::MAX`, as `C` only supports `u32` range.
///
/// # Examples
///
/// ```
/// # use crate::{C, CError, CErrorKind};
/// # use std::convert::TryFrom;
/// assert!(C::try_from(0_u64).is_err());
///
/// let large_value = u64::from(u32::MAX) + 1;
/// assert_eq!(
///     C::try_from(large_value),
///     Err(CError::new(CErrorKind::OutOfRange(large_value.to_string()), "u64", "C"))
/// );
///
/// let value_within_range = 42_u64;
/// assert_eq!(C::try_from(value_within_range), Ok(C::Some(non_zero_u32::new(42).unwrap())));
/// ```
impl TryFrom<u64> for C {
    type Error = CError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "u64",
            "C"
        ))
        } else if value > u32::MAX as u64 {
            Err(CError::new(
                CErrorKind::OutOfRange(value.to_string()),
            "u64",
            "C"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u64",
            "C"
        ))
        }
    }
}

/// Attempts to convert a `u128` into `C`.
///
/// # Examples
///
/// ```
/// use my_crate::C;
/// use std::convert::TryFrom;
///
/// // Successful conversion
/// let c = C::try_from(42u128);
/// assert!(c.is_ok());
///
/// // Zero value, which is an error case
/// let c = C::try_from(0u128);
/// assert!(c.is_err());
///
/// // Value too large for `C`, which is an error case
/// let c = C::try_from(u128::MAX);
/// assert!(c.is_err());
/// ```
///
/// # Errors
///
/// This will return an `Err` if:
///
/// - The value is zero, as `C` cannot represent a zero value.
/// - The value exceeds the maximum value that can be represented by a `u32`.
impl TryFrom<u128> for C {
    type Error = CError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "u128",
            "C"
        ))
        } else if value > u32::MAX as u128 {
            Err(CError::new(
                CErrorKind::OutOfRange(value.to_string()),
            "u128",
            "C"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u128",
            "C"
        ))
        }
    }
}

/// Fallible conversion of a `usize` value into a `C` instance.
///
/// # Examples
///
/// ```
/// # use crate::{C, CError, CErrorKind};
/// # use std::convert::TryFrom;
/// let positive_value = 42_isize;
/// assert!(C::try_from(positive_value).is_ok());
///
/// let negative_value = -42_isize;
/// assert_eq!(
///     C::try_from(negative_value).unwrap_err().kind,
///     CErrorKind::NegativeValue
/// );
///
/// let zero_value = 0_isize;
/// assert_eq!(
///     C::try_from(zero_value).unwrap_err().kind,
///     CErrorKind::Zero
/// );
/// ```
///
/// # Errors
///
/// Returns an `Err` containing a `CError` if:
///
/// - The value is zero, yielding `CErrorKind::Zero`.
/// - The value exceeds the maximum for `u32`, yielding `CErrorKind::OutOfRange`.
///
/// # Notes
///
/// The `usize` type varies in size depending on the target architecture:
/// 32 bits on x86 and 64 bits on x86_64. This implementation ensures that
/// a `usize` value within the range of `u32` can be safely converted to a `C`.
#[cfg(target_pointer_width = "32")]
impl TryFrom<usize> for C {
    type Error = CError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "usize",
            "C"
            ))
        } else if value > u32::MAX as usize {
            Err(CError::new(
                CErrorKind::OutOfRange(format!("Value {} is out of range for C", value)),
            "usize",
            "C"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "usize",
                "C"
                ))
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl TryFrom<usize> for C {
    type Error = CError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(CError::new(
                CErrorKind::Zero,
            "usize",
            "C"
            ))
        } else if value > u32::MAX as usize {
            Err(CError::new(
                CErrorKind::OutOfRange(format!("Value {} is out of range for C", value)),
            "usize",
            "C"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(C::Some)
                .ok_or_else(|| CError::new(
                    CErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "usize",
                "C"
                ))
        }
    }
}

impl core::str::FromStr for C {
    type Err = CError;

    /// Converts a string slice to `C`.
    ///
    /// # Errors
    ///
    /// Returns an error if the string does not represent a valid non-zero u32 value.
    /// This includes zero values, negative values, and values out of range for u32.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{C, CError, CErrorKind};
    ///
    /// let c: Result<C, _> = "42".parse();
    /// assert!(c.is_ok());
    ///
    /// let c: Result<C, _> = "-42".parse();
    /// assert!(matches!(c, Err(CError { kind: CErrorKind::NegativeValue, .. })));
    ///
    /// let c: Result<C, _> = "18446744073709551616".parse();
    /// assert!(matches!(c, Err(CError { kind: CErrorKind::OutOfRange(_), .. })));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(value) => C::new(value),
            Err(_) => {
                // If parsing as u32 failed, try parsing as u64 to determine if it's a range issue
                match s.parse::<u64>() {
                    Ok(value) => {
                        if value > u32::MAX as u64 {
                            Err(CError::new(
                                CErrorKind::OutOfRange("Value out of range for u32".to_string()), "str", "C"))
                        } else {
                            // This branch implies logical error: the value fits within u32, but parse::<u32>() failed.
                            // It should not actually happen in normal circumstances if the input is a valid number.
                            Err(CError::new(
                                CErrorKind::Generic("Invalid number format".to_string()), "str", "C"))
                        }
                    },
                    Err(_) => {
                        // If parsing as u64 also failed, then the string does not represent a valid number.
                        Err(CError::new(
                            CErrorKind::ParseError(s.parse::<u32>().unwrap_err()), "str", "C"))
                    }
                }
            }
        }
    }
}

// Implementation of core::fmt::Display for C

/// `C` represents a positive, non-zero `u32` value.
///
/// # Examples
///
/// ```
/// use crate::C;
///
/// let c = C::Some(nonzero::NonZeroU32::new(42).unwrap());
/// assert_eq!(format!("{}", c), "42");
/// ```
///
/// Attempting to create `C` with a zero or negative value will yield an error:
///
/// ```
/// use crate::{C, CError, CErrorKind};
///
/// let c_result = C::new(0); // or any negative number
/// assert_eq!(c_result.unwrap_err().kind, CErrorKind::Zero); // or `CErrorKind::NegativeValue`
/// ```
impl core::fmt::Display for C {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            C::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future
        }
    }
}

// Implementation of core::fmt::Display for CError

/// `CError` denotes the different kinds of errors that can arise from creating or using `C`.
///
/// # Examples
///
/// Creating `C` with an invalid value:
///
/// ```
/// use crate::{C, CError};
///
/// let c = C::new(0); // Zero is not a valid value for `C`
/// assert!(c.is_err());
/// assert_eq!(format!("{}", c.unwrap_err()), "zero is not allowed");
/// ```
///
/// # Errors
///
/// - `CErrorKind::Zero`: The value provided is zero.
/// - `CErrorKind::NegativeValue`: The value provided is negative.
/// - `CErrorKind::OutOfRange`: The value provided is outside the range of `u32`.
/// - `CErrorKind::ParseError`: The provided string cannot be parsed into a `u32`.
impl core::fmt::Display for CError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            CErrorKind::Unreachable => write!(f, "the Infallible place holder"),
            CErrorKind::InvalidContext => write!(f, "the UniquePtr to Context is null"),
            CErrorKind::NegativeValue => write!(f, "negative value is not allowed"),
            CErrorKind::NoValue => write!(f, "absent value is not allowed"),
            CErrorKind::OutOfRange(s) => write!(f, "{}", s),
            CErrorKind::ParseError(e) => e.fmt(f),
            CErrorKind::Zero => write!(f, "zero is not allowed"),
            CErrorKind::Generic(g) => g.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to simplify the creation of C::Some with NonZeroU32
    fn c_some(value: u32) -> C {
        C::Some(core::num::NonZeroU32::new(value).unwrap())
    }

    fn try_into_c<T>(value: T) -> Result<C, CError>
    where
        C: TryFrom<T>,
        CError: From<<C as TryFrom<T>>::Error>, // Errors via TryFrom are converted to CError
    {
        C::try_from(value).map_err(CError::from)
    }

    #[test]
    fn test_from_impl_for_metric() {
        let c = C::try_from(42).unwrap();
        let metric: Metric = c.into();
        assert!(matches!(metric, Metric::C(_)));
    }

    #[test]
    fn test_c_valid_value() {
        let c = C::new(32);
        assert!(matches!(c, Ok(C::Some(_))));
    }

    #[test]
    fn test_c_new_zero() {
        let c = C::new(0);
        assert!(matches!(c, Err(CError { kind: CErrorKind::Zero, .. })));
    }

    #[test]
    fn test_c_invalid_value() {
        let c = C::new(0);
        assert!(matches!(c, Err(CError { kind: CErrorKind::Zero, .. })));
    }

    #[test]
    fn test_c_from_str_valid() {
        // Parsing a valid value ("1") should succeed.
        let c = "1".parse::<C>();
        assert!(c.is_ok());
        assert_eq!(c.unwrap(), C::Some(core::num::NonZeroU32::new(1).unwrap()));
    }

    #[test]
    fn test_c_from_str_zero() {
        // Trying to parse "0" into C should yield a Zero error.
        let c = "0".parse::<C>();
        assert_eq!(c, Err(CError::new(CErrorKind::Zero, "u32", "C" )));
    }

    #[test]
    fn test_c_from_str_out_of_range() {
        let result = (u64::MAX).to_string().parse::<C>();
        assert!(matches!(result,
            Err(CError { kind: CErrorKind::OutOfRange(_), .. })));
    }

    #[test]
    fn test_c_from_str_invalid() {
        let c = "-1".parse::<C>();
        assert!(c.is_err());
        match c {
            Ok(_) => panic!("Expected error, got Ok(_)"),
            Err(e) => match e.kind {
                CErrorKind::ParseError(_) => (),
                _ => panic!("Expected ParseError, got {:?}", e.kind),
            },
        }
    }

    #[test]
    fn test_c_new() {
        assert!(matches!(try_into_c(1), Ok(C::Some(_))));
    }

    #[test]
    fn test_c_from_i64() {
        assert!(matches!(C::try_from(1i64), Ok(C::Some(_))));
        assert_eq!(C::try_from(0i64), Err(CError::new(
            CErrorKind::Zero, "i64", "C" )));
    }

    #[test]
    fn test_c_from_str_non_numeric() {
        let result = "non_numeric".parse::<C>();
        assert!(matches!(result,
            Err(CError { kind: CErrorKind::ParseError(_), .. })));
    }

    #[test]
    fn test_c_default() {
        assert!(matches!(C::default(), C::Some(_)));
    }

    // Tests for successful conversions
    #[test]
    fn conversion_from_u8_max() {
        let value = u8::MAX;
        assert_eq!(C::try_from(value), Ok(c_some(u32::from(value))));
    }

    #[test]
    fn conversion_from_u16_max() {
        let value = u16::MAX;
        assert_eq!(C::try_from(value), Ok(c_some(u32::from(value))));
    }

    #[test]
    fn conversion_from_u32_max() {
        let value = u32::MAX;
        assert_eq!(C::try_from(value), Ok(c_some(value)));
    }

    // Tests for out-of-range conversions
    #[test]
    fn conversion_from_i8_min() {
        let value = i8::MIN;
        assert_eq!(C::try_from(value), Err(CError::new(
            CErrorKind::NegativeValue, "i8", "C" )));
    }

    #[test]
    fn conversion_from_i16_min() {
        let value = i16::MIN;
        assert_eq!(C::try_from(value), Err(CError::new(
            CErrorKind::NegativeValue, "i16", "C" )));
    }

    #[test]
    fn conversion_from_i32_min() {
        let value = i32::MIN;
        assert_eq!(C::try_from(value), Err(CError::new(
            CErrorKind::NegativeValue, "i32", "C" )));
    }

    #[test]
    fn conversion_from_i64_min() {
        let value = i64::MIN;
        assert_eq!(C::try_from(value), Err(CError::new(
            CErrorKind::NegativeValue, "i64", "C" )));
    }

    #[test]
    fn conversion_from_u64_above_max() {
        let value = u64::from(u32::MAX) + 1;
        assert_eq!(C::try_from(value), Err(CError::new(
            CErrorKind::OutOfRange(value.to_string()), "u64", "C" )));
    }

    #[test]
    fn conversion_from_isize_max() {
        let value = isize::MAX.min(u32::MAX as isize) as u32;
        assert_eq!(C::try_from(value), Ok(c_some(value)));
    }

    #[test]
    fn conversion_from_isize_min() {
        let value = isize::MIN;
        assert_eq!(C::try_from(value), Err(CError::new(
            CErrorKind::NegativeValue, "isize", "C" )));
    }

    #[test]
    fn conversion_from_usize_above_max() {
        let value = u32::MAX as usize + 1;
        assert_eq!(C::try_from(value as u64), Err(CError::new(
            CErrorKind::OutOfRange(value.to_string()), "u64", "C" )));
    }

    // Tests for zero conversions
    #[test]
    fn conversion_from_zero_i32() {
        let value = 0_i32;
        assert_eq!(C::try_from(value), Err(CError::new(
            CErrorKind::Zero, "i32", "C" )));
    }

    #[test]
    fn conversion_from_zero_i64() {
        let value = 0_i64;
        assert_eq!(C::try_from(value), Err(CError::new(
            CErrorKind::Zero, "i64", "C" )));
    }

    // u8 tests
    #[test]
    fn conversion_from_u8_zero() {
        assert_eq!(C::try_from(0_u8), Err(CError::new(
            CErrorKind::Zero, "u8", "C" )));
    }

    // u16 tests
    #[test]
    fn conversion_from_u16_zero() {
        assert_eq!(C::try_from(0_u16), Err(CError::new(
            CErrorKind::Zero, "u16", "C" )));
    }

    #[test]
    fn conversion_from_c_zero() {
        assert_eq!(C::try_from(0_c), Err(CError::new(
            CErrorKind::Zero, "c", "C" )));
    }

    #[test]
    fn conversion_from_c_max() {
        assert_eq!(C::try_from(c::MAX), Ok(c_some(c::MAX as u32)));
    }

    // u16 tests
    #[test]
    fn conversion_from_u16_zero() {
        assert_eq!(C::try_from(0_u16), Err(CError::new(
            CErrorKind::Zero, "u16", "C" )));
    }

    #[test]
    fn conversion_from_u16_max() {
        assert_eq!(C::try_from(u16::MAX), Ok(c_some(u16::MAX as u32)));
    }

    // u32 tests
    #[test]
    fn conversion_from_u32_zero() {
        assert_eq!(C::try_from(0_u32), Err(CError::new(
            CErrorKind::Zero, "u32", "C" )));
    }

    #[test]
    fn conversion_from_u32_max() {
        assert_eq!(C::try_from(u32::MAX), Ok(c_some(u32::MAX)));
    }

    // usize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod usize_tests {
        use super::*;

        #[test]
        fn conversion_from_usize_zero_32bit() {
            assert_eq!(C::try_from(0_usize), Err(CError::new(
                CErrorKind::Zero, "usize", "C" )));
        }

        #[test]
        fn conversion_from_usize_max_32bit() {
            assert_eq!(C::try_from(usize::MAX), Ok(c_some(usize::MAX as u32)));
        }
    }

    // usize tests for 64-bit architecture
    #[cfg(target_pointer_width = "64")]
    mod usize_tests {
        use super::*;

        #[test]
        fn conversion_from_usize_zero_64bit() {
            assert_eq!(C::try_from(0_usize), Err(CError::new(
                CErrorKind::Zero, "usize", "C" )));
        }

        #[test]
        fn conversion_from_usize_max_64bit() {
            assert_eq!(C::try_from(u32::MAX as usize), Ok(c_some(u32::MAX)));
        }
    }

    // i16 tests
    #[test]
    fn conversion_from_i16_zero() {
        assert_eq!(C::try_from(0_i16), Err(CError::new(
            CErrorKind::Zero, "i16", "C" )));
    }

    #[test]
    fn conversion_from_i16_max() {
        assert_eq!(C::try_from(i16::MAX), Ok(c_some(i16::MAX as u32)));
    }

    // i32 tests
    #[test]
    fn conversion_from_i32_zero() {
        assert_eq!(C::try_from(0_i32), Err(CError::new(
            CErrorKind::Zero, "i32", "C" )));
    }

    #[test]
    fn conversion_from_i32_max() {
        assert_eq!(C::try_from(i32::MAX), Ok(c_some(i32::MAX as u32)));
    }

    // isize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_32bit() {
            assert_eq!(C::try_from(0_isize), Err(CError::new(
                CErrorKind::Zero, "isize", "C" )));
        }

        #[test]
        fn conversion_from_isize_max_32bit() {
            assert_eq!(C::try_from(isize::MAX), Ok(c_some(isize::MAX as u32)));
        }

        #[test]
        fn conversion_from_isize_min_32bit() {
            assert_eq!(C::try_from(isize::MIN), Err(CError::new(
                CErrorKind::NegativeValue, "isize", "C" )));
        }
    }

    // isize tests for 64-bit architecture
    #[cfg(target_pointer_width = "64")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_64bit() {
            assert_eq!(C::try_from(0_isize), Err(CError::new(
                CErrorKind::Zero, "isize", "C" )));
        }

        #[test]
        fn conversion_from_isize_max_64bit() {
            let value = isize::MAX.min(u32::MAX as isize) as u32;
            assert_eq!(C::try_from(value), Ok(c_some(value)));
        }

        #[test]
        fn conversion_from_isize_min_64bit() {
            assert_eq!(C::try_from(isize::MIN), Err(CError::new(
                CErrorKind::NegativeValue, "isize", "C" )));
        }
    }

    // Successful conversion tests
    #[test]
    fn test_new_success() {
        assert_eq!(try_into_c(1u16), Ok(C::Some(core::num::NonZeroU32::new(1).unwrap())));
        assert_eq!(try_into_c(1u32), Ok(C::Some(core::num::NonZeroU32::new(1).unwrap())));
        // Add more tests for u64, usize if within u32 range, and all i16, i32, i64, isize within range
    }
    
    // Error case: zero
    #[test]
    fn test_new_zero() {
        assert_eq!(try_into_c(0u32), Err(CError::new(
            CErrorKind::Zero, "u32", "C" )));
        // Add more tests for other types with value zero
    }

    // Error case: negative value
    #[test]
    fn test_new_negative() {
        assert!(matches!(try_into_c(-1i32), Err(CError { kind: CErrorKind::NegativeValue, .. })));
    }

    // Error case: out of range
    #[test]
    fn test_new_out_of_range() {
        assert!(matches!(try_into_c(u64::MAX),
            Err(CError { kind: CErrorKind::OutOfRange(_), .. })));
        // Add more tests for other types with values out of u32 range
    }

    #[test]
    fn test_new_usize_isize_arch_dependent() {
        let max_usize: usize = u32::MAX as usize;
        assert_eq!(try_into_c(max_usize), Ok(C::Some(core::num::NonZeroU32::new(max_usize as u32).unwrap())));

        let max_usize_plus_one: usize = (u32::MAX as usize).wrapping_add(1);
        assert!(matches!(try_into_c(max_usize_plus_one),
            Err(CError { kind: CErrorKind::OutOfRange(_), .. })));

        if cfg!(target_pointer_width = "64") {
            let max_isize: isize = u32::MAX as isize;
            assert_eq!(try_into_c(max_isize), Ok(C::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));

            let max_isize_plus_one: isize = (u32::MAX as isize).wrapping_add(1);
            assert!(matches!(try_into_c(max_isize_plus_one),
                Err(CError { kind: CErrorKind::OutOfRange(_), .. })));

            let min_isize_minus_one: isize = (i32::MIN as isize).wrapping_sub(1);
            assert!(matches!(try_into_c(min_isize_minus_one),
                Err(CError { kind: CErrorKind::NegativeValue, .. })));
        } else if cfg!(target_pointer_width = "32") {
            // For 32-bit architectures, isize max would be within range
            let max_isize: isize = isize::MAX;
            assert_eq!(try_into_c(max_isize), Ok(C::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));
        }
    }

}
