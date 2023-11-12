use core::convert::TryFrom;
use std::num::ParseIntError;
use std::fmt;
use std::convert::{Infallible, TryInto};

use crate::prelude::*;

/// Represents the Hensel lifting degree `r` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `r` typically refers to the Hensel lifting degree.
/// This parameter affects the encoding of integers into polynomials and plays a crucial role in
/// the efficiency of operations and the capacity of the ciphertext.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `r`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `r` often depends on the desired balance between the complexity of operations
/// and the noise growth in ciphertexts. Users should refer to HElib's official documentation or relevant
/// publications for detailed guidelines on selecting `r`.
///
/// # Example
///
/// ```
/// # use your_crate_name::R;  // Replace `your_crate_name` with the name of your crate
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
/// Returns an `MError` with the kind `OutOfRange` if `self` is not a `Some`,
/// indicating the number was zero or never present.
/// The error specifies the conversion attempt from "R" to "u32".
impl crate::bgv::ToU32<MError> for R {
    fn to_u32(&self) -> Result<u32, MError> {
        match self {
            R::Some(non_zero_u32) => Ok(non_zero_u32.get()),
            _ => Err(MError { kind: MErrorKind::OutOfRange(format!("Value {:?} is out of range for R", self)), from: "R", to: "u32" })
        }
    }
}

/// Returns the default value for `R`.
///
/// This is the smallest non-zero `u32` value that is permitted within `R`, typically `4095`.
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
/// assert_eq!(r.unwrap().get(), 4095);
/// ```
impl Default for R {
    fn default() -> Self {
        R::Some(core::num::NonZeroU32::new(4095).expect("4095 is a valid non-zero u32 value."))
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
impl He for R {
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
