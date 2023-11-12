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
/// Returns an `MError` with the kind `OutOfRange` if `self` is not a `Some`,
/// meaning the number was zero or never there.
/// The error details where the problem happened: from "u32" to "Bits".
impl crate::bgv::ToU32<MError> for Bits {
    fn to_u32(&self) -> Result<u32, MError> {
        match self {
            Bits::Some(non_zero_u32) => Ok(non_zero_u32.get()),
            _ => Err(MError {
                kind: MErrorKind::OutOfRange(format!("Value {} is out of range for Bits", self)),
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

/// Represents a single bit within a larger cryptographic framework.
///
/// The `Bits` type encapsulates the notion of a single bit and is used to
/// build up more complex cryptographic structures.
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
