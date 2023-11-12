use core::convert::TryFrom;
use std::num::ParseIntError;
use std::fmt;
use std::convert::{Infallible, TryInto};

use crate::prelude::*;

/// Represents the plaintext modulus parameter `p` in BGV.
///
/// In HElib's BGV encryption scheme, the parameter `p` defines the plaintext modulus.
/// This parameter is crucial in defining the structure of the plaintext space and affects
/// the efficiency and security of the encryption scheme.
///
/// ## Range in this FFI Implementation:
/// This FFI implementation accepts a limited range of values for `p`. Currently, the type
/// uses `NonZeroU32`. This provides a range between 1 and 4,294,967,295 (both inclusive), excluding the value zero.
///
/// ## Range in HElib:
/// In HElib, the choice of `p` is a significant factor in the setup of the encryption scheme.
/// It plays a crucial role in the noise growth during computations and thereby affects the
/// overall depth of computation and security. Users should refer to HElib's official documentation
/// or relevant academic publications for detailed guidelines on selecting `p`.
///
/// # Example
///
/// ```
/// # use your_crate_name::P;  // Replace `your_crate_name` with the name of your crate
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
    /// Constructs a new `PError` related to BGV scheme operations.
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
impl crate::bgv::ToU32<PError> for P {
    fn to_u32(&self) -> Result<u32, PError> {
        match self {
            P::Some(non_zero_u32) => Ok(non_zero_u32.get()),
            _ => Err(PError { kind: PErrorKind::OutOfRange(format!("Value {} is out of range for P", self)), from: "u32", to: "P" })
        }
    }
}

/// Returns the default value for `P`.
///
/// This is the smallest non-zero `u32` value that is permitted within `P`, namely `4095`.
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
/// assert_eq!(p.unwrap().get(), 4095);
/// ```
impl Default for P {
    fn default() -> Self {
        P::Some(core::num::NonZeroU32::new(4095).expect("4095 is a valid non-zero u32 value."))
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
