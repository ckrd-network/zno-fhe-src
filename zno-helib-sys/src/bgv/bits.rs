use core::convert::TryFrom;
use std::num::ParseIntError;
use std::fmt;
use std::convert::{Infallible, TryInto};

use crate::prelude::*;

/// Represents the bit-size parameter `bits` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `bits` typically refers to the bit-size of the modulus.
/// The size of this modulus affects the security and efficiency of the cryptographic operations.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `bits`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `bits` often depends on a trade-off between security and performance.
/// Larger bit-sizes generally offer more security but might be less efficient in terms of computation.
/// Users should refer to HElib's official documentation or relevant publications for detailed guidelines on selecting `bits`.
///
/// # Example
///
/// ```
/// # use your_crate_name::Bits;  // Replace `your_crate_name` with the name of your crate
/// let bits = Bits::new(32).expect("Failed to create Bits");
/// assert_eq!(bits.to_string(), "32");
/// ```
///
/// A non-zero unsigned 32-bit integer representing bit size.
///
/// # Examples
///
/// ```
/// # use your_crate::Bits;
/// let bits = Bits::Some(non_zero_u32::new(128).unwrap());
///
/// assert_eq!(bits, Bits::Some(128));
/// ```
#[derive(Debug, PartialEq)]
pub enum Bits {
    Some(core::num::NonZeroU32),
}

/// An error related to `Bits` operations.
///
/// This error encapsulates various issues that can arise when
/// working with `Bits`, such as conversion errors or invalid values.
///
/// # Examples
///
/// Creating an error due to a negative value:
///
/// ```
/// # use your_crate::{BitsError, BitsErrorKind};
/// let error = BitsError::new(BitsErrorKind::NegativeValue, "i32", "Bits");
///
/// assert_eq!(error.kind, BitsErrorKind::NegativeValue);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct BitsError {
    pub kind: BitsErrorKind,
    pub from: &'static str,
    pub to: &'static str,
}

impl BitsError {
    /// Constructs a new `BitsError`.
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
    /// # use your_crate::{BitsError, BitsErrorKind};
    /// let error = BitsError::new(BitsErrorKind::Zero, "usize", "Bits");
    ///
    /// assert_eq!(error.kind, BitsErrorKind::Zero);
    /// ```
    pub fn new(kind: BitsErrorKind, from: &'static str, to: &'static str) -> Self {
        BitsError { kind, from, to }
    }
}

/// The specific kind of error that `BitsError` can represent.
///
/// # Variants
///
/// * `InvalidContext` - The context in which `Bits` is used is invalid.
/// * `Unreachable` - An unreachable state, indicating a bug.
/// * `NegativeValue` - Attempted to create `Bits` from a negative value.
/// * `NoValue` - A required value for `Bits` was not provided.
/// * `OutOfRange` - A value was out of the valid range for `Bits`.
/// * `ParseError` - Failed to parse a string as `Bits`.
/// * `Zero` - Attempted to create `Bits` with a value of zero.
/// * `Generic` - A generic error, use when none of the others apply.
///
/// # Examples
///
/// ```
/// # use your_crate::{BitsError, BitsErrorKind};
/// let error = BitsError::new(BitsErrorKind::OutOfRange("too large".into()), "u128", "Bits");
///
/// assert_eq!(error.kind, BitsErrorKind::OutOfRange("too large".into()));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum BitsErrorKind {
    InvalidContext,
    Unreachable,
    NegativeValue,
    NoValue,
    OutOfRange(String),
    ParseError(ParseIntError),
    Zero,
    Generic(String),
}

/// Constructs a new `Bits` from a given value.
///
/// This function attempts to create an `Bits` from `value`. It relies on
/// `TryFrom<T>` to do so. If the conversion fails, it returns a `BitsError`.
///
/// # Examples
///
/// ```
/// # use your_crate::Bits;
/// let bits = Bits::new(128);
/// assert!(bits.is_ok());
///
/// let bits = Bits::new(-1);
/// assert!(bits.is_err());
/// ```
///
/// # Errors
///
/// Returns a `BitsError` if:
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
/// See `TryFrom` and `BitsError` for more details.
impl Bits {
    pub fn new<T>(value: T) -> Result<Self, BitsError>
    where
        Self: TryFrom<T, Error = BitsError>,
        T: num_traits::ToPrimitive + std::cmp::PartialOrd + std::fmt::Display + Copy + std::fmt::Debug,
    {
        Bits::try_from(value).map_err(BitsError::from)
    }
}

/// Convert `Bits` to `u32`.
///
/// `Bits` holds a number that cannot be zero. This function extracts that number
/// if it exists.
///
/// # Errors
///
/// Returns an `BitsError` with the kind `OutOfRange` if `self` is not a `Some`,
/// meaning the number was zero or never there.
/// The error details where the problem happened: from "u32" to "Bits".
impl crate::bgv::ToU32<BitsError> for Bits {
    fn to_u32(&self) -> Result<u32, BitsError> {
        match self {
            Bits::Some(non_zero_u32) => Ok(non_zero_u32.get()),
            _ => Err(BitsError {
                kind: BitsErrorKind::OutOfRange(format!("Value {} is out of range for Bits", self)),
                from: "u32",
                to: "Bits"
            })
        }
    }
}

/// Returns the default value for `Bits`.
///
/// This is the smallest non-zero `u32` value that is permitted within `Bits`, namely `4095`.
/// It is a constant, not arbitrary, chosen with purpose.
///
/// # Panics
///
/// This function will panic if the default value cannot be represented as a `NonZeroU32`.
/// Such a panic is not a concern in practical use; The default must be a valid non-zero `u32` value.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let bits = Bits::default();
/// assert_eq!(bits.unwrap().get(), 4095);
/// ```
impl Default for Bits {
    fn default() -> Self {
        Bits::Some(core::num::NonZeroU32::new(4095).expect("4095 is a valid non-zero u32 value."))
    }
}

impl std::error::Error for BitsError {}

/// Converts an `std::io::Error` to `BitsError`.
///
/// # Examples
///
/// ```
/// use std::fs::File;
/// use std::io::{self, Read};
/// use crate::BitsError;
///
/// fn read_file() -> Result<(), BitsError> {
///     let mut file = File::open("m.txt").map_err(BitsError::from)?;
///     let mut contents = String::new();
///     file.read_to_string(&mut contents).map_err(BitsError::from)?;
///     Ok(())
/// }
/// ```
///
/// # Errors
///
/// Returns `BitsError::Generic` containing the error message from `std::io::Error`.
impl From<std::io::Error> for BitsError {
    fn from(e: std::io::Error) -> BitsError {
        BitsError::new(
            BitsErrorKind::Generic(e.to_string()),
            "Error",
            "BitsError"
        )
    }
}

/// Converts a `ParseIntError` to `BitsError`.
///
/// # Arguments
///
/// * `error` - The parse error encountered.
///
/// # Returns
///
/// Returns an `BitsError` with a `ParseError` kind, indicating a parsing failure.
impl From<std::num::ParseIntError> for BitsError {
    fn from(error: std::num::ParseIntError) -> Self {
        BitsError::new(
            BitsErrorKind::ParseError(error),
            "ParseIntError",
            "BitsError"
        )
    }
}

/// Converts from `Infallible` to `BitsError`.
///
/// Since `Infallible` implies no error can occur, this conversion
/// yields a variant representing an unreachable state. It should not
/// be possible for this code to run.
///
/// # Examples
///
/// ```
/// use std::convert::Infallible;
/// use crate::BitsError;
///
/// // Example of infallible conversion, which should not occur:
/// let error: BitsError = Infallible.into();
/// // Assertions about the error kind can be made here if necessary
/// ```
impl From<Infallible> for BitsError {
    fn from(_: Infallible) -> Self {
        BitsError::new(
            BitsErrorKind::Unreachable,
            "Infallible",
            "BitsError"
        )
    }
}

/// Returns the schema type.
///
/// This method declares the homomorphic encryption schema for the implementing
/// type.
/// It is straightforward and utilitarian, reflecting the singular
/// purpose of the `Bits` type within the cryptographic framework.
///
/// # Examples
///
/// ```
/// let bits = Bits::default();
/// assert_eq!(bits.schema(), Schema::Bgv);
/// ```
impl He for Bits {
    fn schema(&self) -> Schema {
        Schema::Bgv
    }
}

/// Converts `Bits` into a `Metric`.
///
/// # Examples
///
/// ```
/// let bits = Bits::new(1i64);
/// let metric: Metric = bits.into();
/// ```
impl Into<Metric> for Bits {
    fn into(self) -> Metric {
        Metric::Bits(self)
    }
}

/// Convert an `i8` to `Bits`.
///
/// # Errors
///
/// Returns `BitsError` in these cases:
///
/// - If `value` is zero, `BitsErrorKind::Zero` is returned.
/// - If `value` is negative, `BitsErrorKind::NegativeValue` is returned.
/// - If `NonZeroU32` cannot be created, `BitsErrorKind::Generic` is returned.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{Bits, BitsError, BitsErrorKind};
///
/// let bits = Bits::try_from(5i8);
/// assert_eq!(bits.is_ok(), true);
///
/// let bits = Bits::try_from(0i8);
/// assert!(matches!(bits, Err(BitsError { kind: BitsErrorKind::Zero, .. })));
///
/// let bits = Bits::try_from(-1i8);
/// assert!(matches!(bits, Err(BitsError { kind: BitsErrorKind::NegativeValue, .. })));
/// ```
impl TryFrom<i8> for Bits {
    type Error = BitsError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
                "i8",
                "Bits"
            ))
        } else if value < 0 {
            Err(BitsError::new(
                BitsErrorKind::NegativeValue,
                "i8",
                "Bits"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "i8",
                    "Bits"
                ))
        }
    }
}

/// Convert an `i16` to `Bits`.
///
/// # Errors
///
/// Returns `BitsError` for the following reasons:
/// - The input is zero. No place for nothing.
/// - The value is negative. It's wrong.
/// - Cannot make `NonZeroU32` from `i16`. It fails silently.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{Bits, BitsError, BitsErrorKind};
/// let positive = Bits::try_from(5i16);
/// assert!(positive.is_ok());
///
/// let zero = Bits::try_from(0i16);
/// assert_eq!(zero.unwrap_err().kind(), &BitsErrorKind::Zero);
///
/// let negative = Bits::try_from(-1i16);
/// assert_eq!(negative.unwrap_err().kind(), &BitsErrorKind::NegativeValue);
/// ```
impl TryFrom<i16> for Bits {
    type Error = BitsError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
                "i16",
                "Bits"
            ))
        } else if value < 0 {
            Err(BitsError::new(
                BitsErrorKind::NegativeValue,
                "i16",
                "Bits"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                    "i16",
                    "Bits"
                ))
        }
    }
}
/// Convert an `i32` to `Bits`.
///
/// # Errors
///
/// Returns `BitsError` in the following cases:
///
/// - If `value` is zero, `BitsError` signifies an attempt to create `Bits` from nothing.
/// - If `value` is negative, `BitsError` signifies an attempt to invert the natural order.
/// - If unable to represent `value` as `NonZeroU32`, `BitsError` indicates a failure in creation.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{Bits, BitsError, BitsErrorKind};
/// assert_eq!(Bits::try_from(5), Ok(Bits::Some(nonzero::NonZeroU32::new(5).unwrap())));
/// assert!(matches!(Bits::try_from(0), Err(BitsError::new(BitsErrorKind::Zero, "i32", "Bits"))));
/// assert!(matches!(Bits::try_from(-1), Err(BitsError::new(BitsErrorKind::NegativeValue, "i32", "Bits"))));
/// ```
impl TryFrom<i32> for Bits {
    type Error = BitsError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
            "i32",
            "Bits"
        ))
        } else if value < 0 {
            Err(BitsError::new(
                BitsErrorKind::NegativeValue,
            "i32",
            "Bits"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i32",
            "Bits"
        ))
        }
    }
}

/// Convert an `i64` to `Bits`.
///
/// # Errors
///
/// Returns `BitsError` if:
///
/// - The value is zero (zero is not allowed).
/// - The value is negative (negatives are not allowed).
/// - The value exceeds `u32`'s maximum (too large for `Bits`).
///
/// # Examples
///
/// ```
/// use std::convert::TryFrom;
/// use crate::{Bits, BitsError};
///
/// let value = 42i64;
/// let bits = Bits::try_from(value);
/// assert!(bits.is_ok());
///
/// let zero = 0i64;
/// let bits = Bits::try_from(zero);
/// assert!(matches!(bits, Err(BitsError { .. })));
/// ```
impl TryFrom<i64> for Bits {
    type Error = BitsError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
            "i64",
            "Bits"
        ))
        } else if value < 0 {
            Err(BitsError::new(
                BitsErrorKind::NegativeValue,
            "i64",
            "Bits"
        ))
        } else if value > u32::MAX as i64 {
            Err(BitsError::new(
                BitsErrorKind::OutOfRange(value.to_string()),
            "i64",
            "Bits"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i64",
            "Bits"
        ))
        }
    }
}

/// Converts an `i128` to `Bits`.
///
/// # Errors
///
/// Returns `Err` with `BitsError` if:
///
/// - Value is `0` (`BitsErrorKind::Zero`).
/// - Value is negative (`BitsErrorKind::NegativeValue`).
/// - Value exceeds `u32::MAX` (`BitsErrorKind::OutOfRange`).
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{Bits, BitsError, BitsErrorKind};
/// let value = 42i128;
/// let bits = Bits::try_from(value);
/// assert_eq!(bits.unwrap(), Bits::Some(NonZeroU32::new(42).unwrap()));
///
/// let zero = 0i128;
/// let bits = Bits::try_from(zero);
/// assert_eq!(bits.unwrap_err().kind, BitsErrorKind::Zero);
///
/// let negative = -1i128;
/// let bits = Bits::try_from(negative);
/// assert_eq!(bits.unwrap_err().kind, BitsErrorKind::NegativeValue);
///
/// let too_large = i128::from(u32::MAX) + 1;
/// let bits = Bits::try_from(too_large);
/// assert_eq!(bits.unwrap_err().kind, BitsErrorKind::OutOfRange);
/// ```
impl TryFrom<i128> for Bits {
    type Error = BitsError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
            "i128",
            "Bits"
        ))
        } else if value < 0 {
            Err(BitsError::new(
                BitsErrorKind::NegativeValue,
            "i128",
            "Bits"
        ))
        } else if value > u32::MAX as i128 {
            Err(BitsError::new(
                BitsErrorKind::OutOfRange(value.to_string()),
            "i128",
            "Bits"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "i128",
            "Bits"
        ))
        }
    }
}

/// Fallible conversion of an `isize` value into a `Bits` instance.
///
/// # Examples
///
/// ```
/// # use crate::{Bits, BitsError, BitsErrorKind};
/// # use std::convert::TryFrom;
/// let positive_value = 42_isize;
/// assert!(Bits::try_from(positive_value).is_ok());
///
/// let negative_value = -42_isize;
/// assert_eq!(
///     Bits::try_from(negative_value).unwrap_err().kind,
///     BitsErrorKind::NegativeValue
/// );
///
/// let zero_value = 0_isize;
/// assert_eq!(
///     Bits::try_from(zero_value).unwrap_err().kind,
///     BitsErrorKind::Zero
/// );
/// ```
///
/// # Errors
///
/// Returns an `Err` containing a `BitsError` if:
///
/// - The value is zero, yielding `BitsErrorKind::Zero`.
/// - The value is negative, yielding `BitsErrorKind::NegativeValue`.
/// - The value exceeds the maximum for `u32`, yielding `BitsErrorKind::OutOfRange`.
///
/// # Notes
///
/// The `isize` type varies in size depending on the target architecture:
/// 32 bits on x86 and 64 bits on x86_64. This implementation ensures that
/// an `isize` value within the range of `u32` can be safely converted to `Bits`.
#[cfg(target_pointer_width = "32")]
impl TryFrom<isize> for Bits {
    type Error = BitsError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
            "isize",
            "Bits"
            ))
        } else if value < 0 {
            Err(BitsError::new(
                BitsErrorKind::NegativeValue,
            "isize",
            "Bits"
            ))
        } else if value > u32::MAX as isize {
            Err(BitsError::new(
                BitsErrorKind::OutOfRange(format!("Value {} is out of range for Bits", value)),
                "isize",
                "Bits"))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "isize",
                "Bits"
                ))
        }
    }
}
#[cfg(target_pointer_width = "64")]
impl TryFrom<isize> for Bits {
    type Error = BitsError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
            "isize",
            "Bits"
            ))
        } else if value < 0 {
            Err(BitsError::new(
                BitsErrorKind::NegativeValue,
            "isize",
            "Bits"
            ))
        } else if value > u32::MAX as isize {
            Err(BitsError::new(
                BitsErrorKind::OutOfRange(format!("Value {} is out of range for Bits", value)),
            "isize",
            "Bits"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "isize",
                "Bits"
                ))
        }
    }
}

/// Attempts to convert a `u8` to `Bits`.
///
/// Fails if input is zero. Non-zero values are safely converted and encapsulated.
///
/// # Examples
///
/// Success:
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::Bits;
/// let value: u8 = 5;
/// assert!(Bits::try_from(value).is_ok());
/// ```
///
/// Failure (zero):
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{Bits, BitsError, BitsErrorKind};
/// let value: u8 = 0;
/// assert_eq!(Bits::try_from(value), Err(BitsError::new(BitsErrorKind::Zero, "u8", "Bits")));
/// ```
///
/// # Errors
///
/// Returns `BitsError` if:
///
/// - Input is zero (`BitsErrorKind::Zero`).
/// - `NonZeroU32` creation fails, unlikely with valid `u8` inputs (`BitsErrorKind::Generic`).
impl TryFrom<u8> for Bits {
    type Error = BitsError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
            "u8",
            "Bits"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u8",
            "Bits"
        ))
        }
    }
}

/// Attempts to create `Bits` from `u16`.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// use std::convert::TryFrom;
/// use your_module::{Bits, BitsError};
///
/// let value = 5u16;
/// let bits = Bits::try_from(value);
///
/// assert!(bits.is_ok());
///
/// let value = 0u16;
/// let bits = Bits::try_from(value);
///
/// assert!(matches!(bits, Err(BitsError { kind: BitsErrorKind::Zero, .. })));
/// ```
///
/// # Errors
///
/// Returns `BitsError` if:
///
/// - Value is zero (`BitsErrorKind::Zero`).
/// - Creation of `NonZeroU32` fails internally, though unlikely (`BitsErrorKind::Generic`).
impl TryFrom<u16> for Bits {
    type Error = BitsError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
            "u16",
            "Bits"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u16",
            "Bits"
        ))
        }
    }
}

/// Attempts to create an `Bits` from a `u32`.
///
/// # Errors
///
/// Returns an `Err` if `value` is zero, as `Bits` cannot be zero.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// # use crate::Bits;
/// # use std::convert::TryFrom;
/// let bits = Bits::try_from(42u32);
/// assert!(bits.is_ok());
///
/// let bits = Bits::try_from(0u32);
/// assert!(bits.is_err());
/// ```
impl TryFrom<u32> for Bits {
    type Error = BitsError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
            "u32",
            "Bits"
        ))
        } else {
            core::num::NonZeroU32::new(value)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u32",
            "Bits"
        ))
        }
    }
}

/// Attempts to convert a `u64` to `Bits`.
///
/// # Errors
///
/// Returns `BitsError` if:
///
/// - The value is `0` (as `Bits` cannot be zero).
/// - The value exceeds `u32::MAX`, as `Bits` only supports `u32` range.
///
/// # Examples
///
/// ```
/// # use crate::{Bits, BitsError, BitsErrorKind};
/// # use std::convert::TryFrom;
/// assert!(Bits::try_from(0_u64).is_err());
///
/// let large_value = u64::from(u32::MAX) + 1;
/// assert_eq!(
///     Bits::try_from(large_value),
///     Err(BitsError::new(BitsErrorKind::OutOfRange(large_value.to_string()), "u64", "Bits"))
/// );
///
/// let value_within_range = 42_u64;
/// assert_eq!(Bits::try_from(value_within_range), Ok(Bits::Some(non_zero_u32::new(42).unwrap())));
/// ```
impl TryFrom<u64> for Bits {
    type Error = BitsError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
            "u64",
            "Bits"
        ))
        } else if value > u32::MAX as u64 {
            Err(BitsError::new(
                BitsErrorKind::OutOfRange(value.to_string()),
            "u64",
            "Bits"
        ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u64",
            "Bits"
        ))
        }
    }
}

// Implementation for u128
/// Convert a `u128` to `Bits`.
///
/// # Errors
///
/// Returns `BitsError` in the following cases:
///
/// - If `value` is zero, `BitsError` signifies an attempt to create a `Bits` from nothing.
/// - If `value` exceeds `u64::MAX`, `BitsError` indicates the value is too large for `Bits`.
///
/// # Examples
///
/// ```
/// # use core::convert::TryFrom;
/// # use crate::{Bits, BitsError, BitsErrorKind};
/// assert_eq!(Bits::try_from(5u128), Ok(Bits::Some(nonzero::NonZeroU32::new(5).unwrap())));
/// assert!(matches!(Bits::try_from(0u128), Err(BitsError::new(BitsErrorKind::Zero, "u128", "Bits"))));
/// assert!(matches!(Bits::try_from(u128::MAX), Err(BitsError::new(BitsErrorKind::OutOfRange("u128::MAX".to_string()), "u128", "Bits"))));
/// ```
impl TryFrom<u128> for Bits {
    type Error = BitsError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
                "u128",
                "Bits",
            ))
        } else if value > u64::MAX as u128 {
            Err(BitsError::new(
                BitsErrorKind::OutOfRange(value.to_string()),
                "u128",
                "Bits",
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
            "u128",
            "Bits"
        ))
        }
    }
}

/// Fallible conversion of a `usize` value into a `Bits` instance.
///
/// # Examples
///
/// ```
/// # use crate::{Bits, BitsError, BitsErrorKind};
/// # use std::convert::TryFrom;
/// let positive_value = 42_usize;
/// assert!(Bits::try_from(positive_value).is_ok());
///
/// let zero_value = 0_usize;
/// assert_eq!(
///     Bits::try_from(zero_value).unwrap_err().kind,
///     BitsErrorKind::Zero
/// );
/// ```
///
/// # Errors
///
/// Returns an `Err` containing a `BitsError` if:
///
/// - The value is zero, yielding `BitsErrorKind::Zero`.
/// - The value exceeds the maximum for `u64`, yielding `BitsErrorKind::OutOfRange`.
///
/// # Notes
///
/// The `usize` type varies in size depending on the target architecture:
/// 32 bits on x86 and 64 bits on x86_64. This implementation ensures that
/// a `usize` value within the range of `u64` can be safely converted to a `Bits`.
#[cfg(target_pointer_width = "32")]
impl TryFrom<usize> for Bits {
    type Error = BitsError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
                "usize",
                "Bits"
            ))
        } else if value > u64::MAX as usize {
            Err(BitsError::new(
                BitsErrorKind::OutOfRange(format!("Value {} is out of range for Bits", value)),
                "usize",
                "Bits"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "usize",
                "Bits"
                ))
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl TryFrom<usize> for Bits {
    type Error = BitsError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(BitsError::new(
                BitsErrorKind::Zero,
                "usize",
                "Bits"
            ))
        } else if value > u32::MAX as usize {
            Err(BitsError::new(
                BitsErrorKind::OutOfRange(format!("Value {} is out of range for Bits", value)),
                "usize",
                "Bits"
            ))
        } else {
            core::num::NonZeroU32::new(value as u32)
                .map(Bits::Some)
                .ok_or_else(|| BitsError::new(
                    BitsErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                "usize",
                "Bits"
                ))
        }
    }
}

impl core::str::FromStr for Bits {
    type Err = BitsError;
    /// Converts a string slice to `Bits`.
    ///
    /// # Errors
    ///
    /// Returns an error if the string does not represent a valid non-zero u32 value.
    /// This includes zero values, negative values, and values out of range for u32.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::{Bits, BitsError, BitsErrorKind};
    ///
    /// let bits: Result<Bits, _> = "32".parse();
    /// assert!(bits.is_ok());
    ///
    /// let bits: Result<Bits, _> = "-42".parse();
    /// assert!(matches!(bits, Err(BitsError { kind: BitsErrorKind::NegativeValue, .. })));
    ///
    /// let bits: Result<Bits, _> = "18446744073709551616".parse();
    /// assert!(matches!(bits, Err(BitsError { kind: BitsErrorKind::OutOfRange(_), .. })));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(value) => Bits::new(value),
            Err(_) => {
                // If parsing as u32 failed, try parsing as u64 to determine if it's a range issue
                match s.parse::<u64>() {
                    Ok(value) => {
                        if value > u32::MAX as u64 {
                            Err(BitsError::new(
                                BitsErrorKind::OutOfRange("Value out of range for u32".to_string()), "str","Bits"))
                        } else {
                            // This branch implies logical error: the value fits within u32, but parse::<u32>() failed.
                            // It should not actually happen in normal circumstances if the input is a valid number.
                            Err(BitsError::new(
                                BitsErrorKind::Generic("Invalid number format".to_string()), "str","Bits"))
                        }
                    },
                    Err(_) => {
                        // If parsing as u64 also failed, then the string does not represent a valid number.
                        Err(BitsError::new(
                            BitsErrorKind::ParseError(s.parse::<u32>().unwrap_err()), "str","Bits"))
                    }
                }
            }
        }
    }
}

/// `Bits` represents a positive, non-zero `u64` value.
///
/// # Examples
///
/// ```
/// use crate::Bits;
///
/// let bits = Bits::Some(nonzero::NonZeroU32::new(42).unwrap());
/// assert_eq!(format!("{}", bits), "42");
/// ```
///
/// Attempting to create `Bits` with a zero or negative value will yield an error:
///
/// ```
/// use crate::{Bits, BitsError, BitsErrorKind};
///
/// let bits_result = Bits::new(0); // or any negative number
/// assert_eq!(bits_result.unwrap_err().kind, BitsErrorKind::Zero); // or `BitsErrorKind::NegativeValue`
/// ```
impl core::fmt::Display for Bits {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Bits::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future
        }
    }
}

/// `BitsError` denotes the different kinds of errors that can arise from creating or using `Bits`.
///
/// # Examples
///
/// Creating `Bits` with an invalid value:
///
/// ```
/// use crate::{Bits, BitsError};
///
/// let bits = Bits::new(0); // Zero is not a valid value for `Bits`
/// assert!(bits.is_err());
/// assert_eq!(format!("{}", bits.unwrap_err()), "zero is not allowed");
/// ```
///
/// # Errors
///
/// - `BitsErrorKind::Zero`: The value provided is zero.
/// - `BitsErrorKind::NegativeValue`: The value provided is negative.
/// - `BitsErrorKind::OutOfRange`: The value provided is outside the range of `u64`.
/// - `BitsErrorKind::ParseError`: The provided string cannot be parsed into a `u64`.
impl core::fmt::Display for BitsError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.kind {
            BitsErrorKind::Unreachable => write!(f, "the Infallible place holder"),
            BitsErrorKind::InvalidContext => write!(f, "the UniquePtr to Context is null"),
            BitsErrorKind::NegativeValue => write!(f, "negative value is not allowed"),
            BitsErrorKind::NoValue => write!(f, "absent value is not allowed"),
            BitsErrorKind::OutOfRange(s) => write!(f, "{}", s),
            BitsErrorKind::ParseError(e) => e.fmt(f),
            BitsErrorKind::Zero => write!(f, "zero is not allowed"),
            BitsErrorKind::Generic(g) => g.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to simplify the creation of Bits::Some with NonZeroU32
    fn bits_some(value: u32) -> Bits {
        Bits::Some(core::num::NonZeroU32::new(value).unwrap())
    }

    fn try_into_bits<T>(value: T) -> Result<Bits, BitsError>
    where
        Bits: TryFrom<T>,
        BitsError: From<<Bits as TryFrom<T>>::Error>, // Errors via TryFrom are converted to BitsError
    {
        Bits::try_from(value).map_err(BitsError::from)
    }

    #[test]
    fn test_from_impl_for_bits() {
        let bits = Bits::try_from(42).unwrap();
        assert!(matches!(bits, Bits::Some(_)));
    }

    #[test]
    fn test_bits_valid_value() {
        let bits = Bits::new(32);
        assert!(matches!(bits, Ok(Bits::Some(_))));
    }

    #[test]
    fn test_bits_new_zero() {
        let bits = Bits::new(0);
        assert!(matches!(bits, Err(BitsError { kind: BitsErrorKind::Zero, .. })));
    }

    #[test]
    fn test_bits_invalid_value() {
        let bits = Bits::new(0);
        assert!(matches!(bits, Err(BitsError { kind: BitsErrorKind::Zero, .. })));
    }

    #[test]
    fn test_bits_from_str_valid() {
        let bits = "1".parse::<Bits>();
        assert!(bits.is_ok());
        assert_eq!(bits.unwrap(), Bits::Some(core::num::NonZeroU32::new(1).unwrap()));
    }

    #[test]
    fn test_bits_from_str_zero() {
        let bits = "0".parse::<Bits>();
        assert_eq!(bits, Err(BitsError::new(BitsErrorKind::Zero, "u32", "Bits" )));
    }

    #[test]
    fn test_bits_from_str_out_of_range() {
        let result = (u64::MAX).to_string().parse::<Bits>();
        assert!(matches!(result,
            Err(BitsError { kind: BitsErrorKind::OutOfRange(_), .. })));
    }

    #[test]
    fn test_bits_from_str_invalid() {
        let bits = "-1".parse::<Bits>();
        assert!(bits.is_err());
        match bits {
            Ok(_) => panic!("Expected error, got Ok(_)"),
            Err(e) => match e.kind {
                BitsErrorKind::ParseError(_) => (),
                _ => panic!("Expected ParseError, got {:?}", e.kind),
            },
        }
    }

    #[test]
    fn test_bits_new() {
        assert!(matches!(try_into_bits(1), Ok(Bits::Some(_))));
    }

    #[test]
    fn test_bits_from_i64() {
        assert!(matches!(Bits::try_from(1i64), Ok(Bits::Some(_))));
        assert_eq!(Bits::try_from(0i64), Err(BitsError::new(
            BitsErrorKind::Zero, "i64", "Bits" )));
    }

    #[test]
    fn test_bits_from_str_non_numeric() {
        let result = "non_numeric".parse::<Bits>();
        assert!(matches!(result,
            Err(BitsError { kind: BitsErrorKind::ParseError(_), .. })));
    }

    #[test]
    fn test_bits_default() {
        assert!(matches!(Bits::default(), Bits::Some(_)));
    }

    // Tests for successful conversions
    #[test]
    fn conversion_from_u8_max() {
        let value = u8::MAX;
        assert_eq!(Bits::try_from(value), Ok(bits_some(u32::from(value))));
    }

    #[test]
    fn conversion_from_u16_max() {
        let value = u16::MAX;
        assert_eq!(Bits::try_from(value), Ok(bits_some(u32::from(value))));
    }

    #[test]
    fn conversion_from_u32_max() {
        let value = u32::MAX;
        assert_eq!(Bits::try_from(value), Ok(bits_some(value)));
    }

    // Tests for out-of-range conversions
    #[test]
    fn conversion_from_i8_min() {
        let value = i8::MIN;
        assert_eq!(Bits::try_from(value), Err(BitsError::new(
            BitsErrorKind::NegativeValue, "i8", "Bits" )));
    }

    #[test]
    fn conversion_from_i16_min() {
        let value = i16::MIN;
        assert_eq!(Bits::try_from(value), Err(BitsError::new(
            BitsErrorKind::NegativeValue, "i16", "Bits" )));
    }

    #[test]
    fn conversion_from_i32_min() {
        let value = i32::MIN;
        assert_eq!(Bits::try_from(value), Err(BitsError::new(
            BitsErrorKind::NegativeValue, "i32", "Bits" )));
    }

    #[test]
    fn conversion_from_i64_min() {
        let value = i64::MIN;
        assert_eq!(Bits::try_from(value), Err(BitsError::new(
            BitsErrorKind::NegativeValue, "i64", "Bits" )));
    }

    #[test]
    fn conversion_from_u64_above_max() {
        let value = u64::from(u32::MAX) + 1;
        assert_eq!(Bits::try_from(value), Err(BitsError::new(
            BitsErrorKind::OutOfRange(value.to_string()), "u64", "Bits" )));
    }

    #[test]
    fn conversion_from_isize_max() {
        let value = isize::MAX.min(u32::MAX as isize) as u32;
        assert_eq!(Bits::try_from(value), Ok(bits_some(value)));
    }

    #[test]
    fn conversion_from_isize_min() {
        let value = isize::MIN;
        assert_eq!(Bits::try_from(value), Err(BitsError::new(
            BitsErrorKind::NegativeValue, "isize", "Bits" )));
    }

    #[test]
    fn conversion_from_usize_above_max() {
        let value = u32::MAX as usize + 1;
        assert_eq!(Bits::try_from(value as u64), Err(BitsError::new(
            BitsErrorKind::OutOfRange(value.to_string()), "u64", "Bits" )));
    }

    // Tests for zero conversions
    #[test]
    fn conversion_from_zero_i32() {
        let value = 0_i32;
        assert_eq!(Bits::try_from(value), Err(BitsError::new(
            BitsErrorKind::Zero, "i32", "Bits" )));
    }

    #[test]
    fn conversion_from_zero_i64() {
        let value = 0_i64;
        assert_eq!(Bits::try_from(value), Err(BitsError::new(
            BitsErrorKind::Zero, "i64", "Bits" )));
    }

    // u8 tests
    #[test]
    fn conversion_from_u8_zero() {
        assert_eq!(Bits::try_from(0_u8), Err(BitsError::new(
            BitsErrorKind::Zero, "u8", "Bits" )));
    }

    // u16 tests
    #[test]
    fn conversion_from_u16_zero() {
        assert_eq!(Bits::try_from(0_u16), Err(BitsError::new(
            BitsErrorKind::Zero, "u16", "Bits" )));
    }

    // i8 tests for Bits parameter
    #[test]
    fn conversion_from_i8_zero() {
        assert_eq!(Bits::try_from(0_i8), Err(BitsError::new(
            BitsErrorKind::Zero, "i8", "Bits" )));
    }

    #[test]
    fn conversion_from_i8_max() {
        assert_eq!(Bits::try_from(i8::MAX), Ok(bits_some(i8::MAX as u32)));
    }

    // i16 tests for Bits parameter
    #[test]
    fn conversion_from_i16_zero() {
        assert_eq!(Bits::try_from(0_i16), Err(BitsError::new(
            BitsErrorKind::Zero, "i16", "Bits" )));
    }

    #[test]
    fn conversion_from_i16_max() {
        assert_eq!(Bits::try_from(i16::MAX), Ok(bits_some(i16::MAX as u32)));
    }

    // Tests for usize and isize depend on the architecture of
    // the machine (32-bit or 64-bit).
    // usize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod usize_tests {
        use super::*;

        #[test]
        fn conversion_from_usize_zero_32bit() {
            assert_eq!(Bits::try_from(0_usize), Err(BitsError::new(
                BitsErrorKind::Zero, "usize", "Bits" )));
        }

        #[test]
        fn conversion_from_usize_max_32bit() {
            assert_eq!(Bits::try_from(usize::MAX), Ok(bits_some(usize::MAX as u32)));
        }
    }

    // usize tests for 64-bit architecture
    #[cfg(target_pointer_width = "64")]
    mod usize_tests {
        use super::*;

        #[test]
        fn conversion_from_usize_zero_64bit() {
            assert_eq!(Bits::try_from(0_usize), Err(BitsError::new(
                BitsErrorKind::Zero, "usize", "Bits" )));
        }

        #[test]
        fn conversion_from_usize_max_64bit() {
            assert_eq!(Bits::try_from(u32::MAX as usize), Ok(bits_some(u32::MAX)));
        }
    }

    // isize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_32bit() {
            assert_eq!(Bits::try_from(0_isize), Err(BitsError::new(
                BitsErrorKind::Zero, "isize", "Bits" )));
        }

        #[test]
        fn conversion_from_isize_max_32bit() {
            assert_eq!(Bits::try_from(isize::MAX), Ok(bits_some(isize::MAX as u32)));
        }

        #[test]
        fn conversion_from_isize_min_32bit() {
            assert_eq!(Bits::try_from(isize::MIN), Err(BitsError::new(
                BitsErrorKind::NegativeValue, "isize", "Bits" )));
        }
    }

    // isize tests for 64-bit architecture
    #[cfg(target_pointer_width = "64")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_64bit() {
            assert_eq!(Bits::try_from(0_isize), Err(BitsError::new(
                BitsErrorKind::Zero, "isize", "Bits" )));
        }

        #[test]
        fn conversion_from_isize_max_64bit() {
            let value = isize::MAX.min(u32::MAX as isize) as u32;
            assert_eq!(Bits::try_from(value), Ok(bits_some(value)));
        }

        #[test]
        fn conversion_from_isize_min_64bit() {
            assert_eq!(Bits::try_from(isize::MIN), Err(BitsError::new(
                BitsErrorKind::NegativeValue, "isize", "Bits" )));
        }
    }

    // Successful conversion tests
    #[test]
    fn test_new_success() {
        assert_eq!(try_into_bits(1u8), Ok(Bits::Some(core::num::NonZeroU32::new(1).unwrap())));
        assert_eq!(try_into_bits(1u16), Ok(Bits::Some(core::num::NonZeroU32::new(1).unwrap())));
        assert_eq!(try_into_bits(1u32), Ok(Bits::Some(core::num::NonZeroU32::new(1).unwrap())));
        // Add more tests for u64, usize if within u32 range, and all i8, i16, i32, i64, isize within range
    }

    // Error case: zero
    #[test]
    fn test_new_zero_bits() {
        assert_eq!(try_into_bits(0u32), Err(BitsError::new(
            BitsErrorKind::Zero, "u32", "Bits" )));
        // Add more tests for other types with value zero
    }

    // Error case: negative value
    #[test]
    fn test_new_negative_bits() {
        assert!(matches!(try_into_bits(-1i32), Err(BitsError { kind: BitsErrorKind::NegativeValue, .. })));
    }

    // Error case: out of range
    #[test]
    fn test_new_out_of_range_bits() {
        assert!(matches!(try_into_bits(u64::MAX),
            Err(BitsError { kind: BitsErrorKind::OutOfRange(_), .. })));
        // Add more tests for other types with values out of u32 range
    }

    #[test]
    fn test_new_usize_isize_arch_dependent() {
        let max_usize: usize = u32::MAX as usize;
        assert_eq!(try_into_bits(max_usize), Ok(Bits::Some(core::num::NonZeroU32::new(max_usize as u32).unwrap())));

        let max_usize_plus_one: usize = (u32::MAX as usize).wrapping_add(1);
        assert!(matches!(try_into_bits(max_usize_plus_one),
            Err(BitsError { kind: BitsErrorKind::OutOfRange(_), .. })));

        if cfg!(target_pointer_width = "64") {
            let max_isize: isize = u32::MAX as isize;
            assert_eq!(try_into_bits(max_isize), Ok(Bits::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));

            let max_isize_plus_one: isize = (u32::MAX as isize).wrapping_add(1);
            assert!(matches!(try_into_bits(max_isize_plus_one),
                Err(BitsError { kind: BitsErrorKind::OutOfRange(_), .. })));

            let min_isize_minus_one: isize = (i32::MIN as isize).wrapping_sub(1);
            assert!(matches!(try_into_bits(min_isize_minus_one),
                Err(BitsError { kind: BitsErrorKind::NegativeValue, .. })));
        } else if cfg!(target_pointer_width = "32") {
            // For 32-bit architectures, isize max would be within range
            let max_isize: isize = isize::MAX;
            assert_eq!(try_into_bits(max_isize), Ok(Bits::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));
        }
    }

}
