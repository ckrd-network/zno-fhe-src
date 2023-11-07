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
#[derive(Debug, PartialEq)]
pub enum M {
    Some(core::num::NonZeroU32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MError {
    pub kind: MErrorKind,
}

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

impl M {
    pub fn new<T>(value: T) -> Result<Self, MError>
    where
        Self: TryFrom<T, Error = MError>,
        T: num_traits::ToPrimitive + std::cmp::PartialOrd + std::fmt::Display + Copy + std::fmt::Debug,
    {
        // TryFrom trait implementation for conversion.
        M::try_from(value).map_err(MError::from)
    }
}

impl crate::bgv::ToU32<MError> for M {
    fn to_u32(&self) -> Result<u32, MError> {
        match self {
            M::Some(non_zero_u32) => Ok(non_zero_u32.get()),
            _ => Err(MError {
                kind: MErrorKind::OutOfRange("Value out of range of u32".into()), // A generic error.
            }),
        }
    }
}

/// Provides a default `M` value. A error should never occur, as this is a safe default.
impl Default for M {
    fn default() -> Self {
        M::Some(core::num::NonZeroU32::new(4095).expect("32 is a valid non-zero u32 value."))
    }
}

impl std::error::Error for MError {}

// Implement From for each error type to convert into MError
impl From<std::io::Error> for MError {
    fn from(e: std::io::Error) -> MError {
        MError {
            kind: MErrorKind::Generic(e.to_string()),
        }
    }
}

impl From<std::num::ParseIntError> for MError {
    fn from(error: std::num::ParseIntError) -> Self {
        MError {
            kind: MErrorKind::ParseError(error),
        }
    }
}

impl From<Infallible> for MError {
    fn from(_: Infallible) -> Self {
        // Infallible means the error case will never happen
        // However, return an error variant denoting an unreachable state
        MError {
            kind: MErrorKind::Unreachable
        }
    }
}

impl He for M {
    fn schema(&self) -> Schema {
        Schema::Bgv
    }
}

impl Into<Metric> for M {
    fn into(self) -> Metric {
        Metric::M(self)
    }
}

//
// Implementations for fixed-size signed integers
//
impl TryFrom<i8> for M {
    type Error = MError;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else if value < 0 {
            Err(MError {
                kind: MErrorKind::NegativeValue,
            })
        } else {
            // Directly construct M, since i8 values are within u32 range when positive and non-zero.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}


impl TryFrom<i16> for M {
    type Error = MError;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else if value < 0 {
            Err(MError {
                kind: MErrorKind::NegativeValue,
            })
        } else {
            // Directly construct M, since i16 values are within u32 range when positive and non-zero.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

impl TryFrom<i32> for M {
    type Error = MError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else if value < 0 {
            Err(MError {
                kind: MErrorKind::NegativeValue,
            })
        } else {
            // Directly construct M, since i32 values are within u32 range when positive and non-zero.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

impl TryFrom<i64> for M {
    type Error = MError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else if value < 0 {
            Err(MError {
                kind: MErrorKind::NegativeValue,
            })
        } else if value > u32::MAX as i64 {
            // If the i64 value is greater than the maximum for a u32, it's out of bounds.
            Err(MError {
                kind: MErrorKind::OutOfRange(value.to_string()),
            })
        } else {
            // Safe to convert i64 to u32 because it's within the valid range and non-zero.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

impl TryFrom<i128> for M {
    type Error = MError;

    fn try_from(value: i128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else if value < 0 {
            Err(MError {
                kind: MErrorKind::NegativeValue,
            })
        } else if value > u32::MAX as i128 {
            // If the i128 value is greater than the maximum for a u32, it's out of bounds.
            Err(MError {
                kind: MErrorKind::OutOfRange(value.to_string()),
            })
        } else {
            // Safe to convert i128 to u32 because it's within the valid range and non-zero.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

//
// Implementations for fixed-size unsigned integers
// u8, u16, u32, u64, u128

impl TryFrom<u8> for M {
    type Error = MError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else {
            // Directly construct M, since u8 values are within u32 range when positive and non-zero.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

impl TryFrom<u16> for M {
    type Error = MError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else {
            // Directly construct M, since u16 values are within u32 range when positive and non-zero.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

impl TryFrom<u32> for M {
    type Error = MError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else {
            // Directly construct M, since u32 values are within u32 range when positive and non-zero.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

impl TryFrom<u64> for M {
    type Error = MError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else if value > u32::MAX as u64 {
            // If the u64 value is greater than the maximum for a u32, it's out of bounds.
            Err(MError {
                kind: MErrorKind::OutOfRange(value.to_string()),
            })
        } else {
            // Safe to convert u64 to u32 because it's within the valid range and non-zero.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

impl TryFrom<u128> for M {
    type Error = MError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else if value > u32::MAX as u128 {
            // If the u128 value is greater than the maximum for a u32, it's out of bounds.
            Err(MError {
                kind: MErrorKind::OutOfRange(value.to_string()),
            })
        } else {
            // Safe to convert u128 to u32 because it's within the valid range and non-zero.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

//
// Implementations for pointer-sized signed integers
//
#[cfg(target_pointer_width = "32")]
impl TryFrom<isize> for M {
    type Error = MError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else if value < 0 {
            Err(MError {
                kind: MErrorKind::NegativeValue,
            })
        } else if value > u32::MAX as isize {
            Err(MError {
                kind: MErrorKind::OutOfRange(format!("Value {} is out of range for M", value)),
            })
        } else {
            // It's safe to cast to u32 because we've already checked it's within range.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl TryFrom<isize> for M {
    type Error = MError;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else if value < 0 {
            Err(MError {
                kind: MErrorKind::NegativeValue,
            })
        } else if value > u32::MAX as isize {
            Err(MError {
                kind: MErrorKind::OutOfRange(format!("Value {} is out of range for M", value)),
            })
        } else {
            // It's safe to cast to u32 because we've already checked it's within range.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

//
// Implementations for pointer-sized unsigned integers
//
#[cfg(target_pointer_width = "32")]
impl TryFrom<usize> for M {
    type Error = MError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else if value > u32::MAX as usize {
            Err(MError {
                kind: MErrorKind::OutOfRange(format!("Value {} is out of range for M", value)),
            })
        } else {
            // It's safe to cast to u32 because we've already checked it's within range.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

#[cfg(target_pointer_width = "64")]
impl TryFrom<usize> for M {
    type Error = MError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(MError {
                kind: MErrorKind::Zero,
            })
        } else if value > u32::MAX as usize {
            Err(MError {
                kind: MErrorKind::OutOfRange(format!("Value {} is out of range for M", value)),
            })
        } else {
            // It's safe to cast to u32 because we've already checked it's within range.
            core::num::NonZeroU32::new(value as u32)
                .map(M::Some)
                .ok_or_else(|| MError {
                    kind: MErrorKind::Generic("Failed to create NonZeroU32".to_string()),
                })
        }
    }
}

impl core::str::FromStr for M {
    type Err = MError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u32>() {
            Ok(value) => M::new(value),
            Err(_) => {
                // If parsing as u32 failed, try parsing as u64 to determine if it's a range issue
                match s.parse::<u64>() {
                    Ok(value) => {
                        if value > u32::MAX as u64 {
                            Err(MError { kind: MErrorKind::OutOfRange("Value out of range for u32".to_string()) })
                        } else {
                            // This branch implies logical error: the value fits within u32, but parse::<u32>() failed.
                            // It should not actually happen in normal circumstances if the input is a valid number.
                            Err(MError { kind: MErrorKind::Generic("Invalid number format".to_string()) })
                        }
                    },
                    Err(_) => {
                        // If parsing as u64 also failed, then the string does not represent a valid number.
                        Err(MError { kind: MErrorKind::ParseError(s.parse::<u32>().unwrap_err()) })
                    }
                }
            }
        }
    }
}

impl core::fmt::Display for M {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            M::Some(value) => write!(f, "{}", value),
            // Handle other variants if they are added in the future
        }
    }
}

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

    // Tests for usize and isize that depend on the machine architecture
    fn try_into_m<T>(value: T) -> Result<M, MError>
    where
        M: TryFrom<T>,
        MError: From<<M as TryFrom<T>>::Error>, // Ensures that whatever error TryFrom produces, can be converted to MError
    {
        M::try_from(value).map_err(MError::from)
    }

    #[test]
    fn test_from_impl_for_metric() {
        let m = M::try_from(42).unwrap(); // Valid value for M
        let metric: Metric = m.into(); // Should not fail
        assert!(matches!(metric, Metric::M(_)));
    }

    #[test]
    fn test_m_valid_value() {
        let m = M::new(32);
        assert!(matches!(m, Ok(M::Some(_))));
    }

    #[test]
    fn test_m_new_zero() {
        // Trying to create M with zero should yield a Zero error.
        let m = M::new(0);
        assert_eq!(m, Err(MError { kind: MErrorKind::Zero }));
    }

    #[test]
    fn test_m_invalid_value() {
        let m = M::new(0);
        assert!(matches!(m, Err(MError { kind: MErrorKind::Zero })));
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
        assert_eq!(m, Err(MError { kind: MErrorKind::Zero }));
    }

    #[test]
    fn test_m_from_str_out_of_range() {
        let result = (u64::MAX).to_string().parse::<M>();
        assert!(matches!(result, Err(MError { kind: MErrorKind::OutOfRange(_) })));
    }

    #[test]
    fn test_m_from_str_invalid() {
        // Parsing an invalid value ("-1") should fail.
        let m = "-1".parse::<M>();
        assert!(m.is_err());
        // Check for a specific error if you want
        match m {
            Err(MError { kind: MErrorKind::ParseError(_) }) => (), // This is expected, do nothing
            _ => panic!("Expected ParseError"), // Panic with a custom message in all other cases
        }
    }

    #[test]
    fn test_m_new() {
        assert!(matches!(try_into_m(1), Ok(M::Some(_))));
    }

    #[test]
    fn test_m_from_i64() {
        assert!(matches!(M::try_from(1i64), Ok(M::Some(_))));
        assert_eq!(M::try_from(0i64), Err(MError { kind: MErrorKind::Zero }));
    }

    #[test]
    fn test_m_from_str_non_numeric() {
        let result = "non_numeric".parse::<M>();
        assert!(matches!(result, Err(MError { kind: MErrorKind::ParseError(_) })));
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
        assert_eq!(M::try_from(value), Err(MError { kind: MErrorKind::NegativeValue }));
    }

    #[test]
    fn conversion_from_i16_min() {
        let value = i16::MIN;
        assert_eq!(M::try_from(value), Err(MError { kind: MErrorKind::NegativeValue }));
    }

    #[test]
    fn conversion_from_i32_min() {
        let value = i32::MIN;
        assert_eq!(M::try_from(value), Err(MError { kind: MErrorKind::NegativeValue }));
    }

    #[test]
    fn conversion_from_i64_min() {
        let value = i64::MIN;
        assert_eq!(M::try_from(value), Err(MError { kind: MErrorKind::NegativeValue }));
    }

    #[test]
    fn conversion_from_u64_above_max() {
        let value = u64::from(u32::MAX) + 1;
        assert_eq!(M::try_from(value), Err(MError { kind: MErrorKind::OutOfRange(value.to_string()) }));
    }

    #[test]
    fn conversion_from_isize_max() {
        let value = isize::MAX.min(u32::MAX as isize) as u32;
        assert_eq!(M::try_from(value), Ok(m_some(value)));
    }

    #[test]
    fn conversion_from_isize_min() {
        let value = isize::MIN;
        assert_eq!(M::try_from(value), Err(MError { kind: MErrorKind::NegativeValue }));
    }

    #[test]
    fn conversion_from_usize_above_max() {
        let value = u32::MAX as usize + 1;
        assert_eq!(M::try_from(value as u64), Err(MError { kind: MErrorKind::OutOfRange(value.to_string()) }));
    }

    // Tests for zero conversions
    #[test]
    fn conversion_from_zero_i32() {
        let value = 0_i32;
        assert_eq!(M::try_from(value), Err(MError { kind: MErrorKind::Zero }));
    }

    #[test]
    fn conversion_from_zero_i64() {
        let value = 0_i64;
        assert_eq!(M::try_from(value), Err(MError { kind: MErrorKind::Zero }));
    }

    // u8 tests
    #[test]
    fn conversion_from_u8_zero() {
        assert_eq!(M::try_from(0_u8), Err(MError { kind: MErrorKind::Zero }));
    }

    // u16 tests
    #[test]
    fn conversion_from_u16_zero() {
        assert_eq!(M::try_from(0_u16), Err(MError { kind: MErrorKind::Zero }));
    }

    // i8 tests
    #[test]
    fn conversion_from_i8_zero() {
        assert_eq!(M::try_from(0_i8), Err(MError { kind: MErrorKind::Zero }));
    }

    #[test]
    fn conversion_from_i8_max() {
        assert_eq!(M::try_from(i8::MAX), Ok(m_some(i8::MAX as u32)));
    }

    // i16 tests
    #[test]
    fn conversion_from_i16_zero() {
        assert_eq!(M::try_from(0_i16), Err(MError { kind: MErrorKind::Zero }));
    }

    #[test]
    fn conversion_from_i16_max() {
        assert_eq!(M::try_from(i16::MAX), Ok(m_some(i16::MAX as u32)));
    }

    // Tests for larger types at the boundary of u32::MAX are similar to the above.

    // Comprehensive coverage for usize and isize depend on the architecture of
    // the machine (32-bit or 64-bit).
    // Here are examples for a 64-bit architecture:

    // usize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod usize_tests {
        use super::*;

        #[test]
        fn conversion_from_usize_zero_32bit() {
            assert_eq!(M::try_from(0_usize), Err(MError { kind: MErrorKind::Zero }));
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
            assert_eq!(M::try_from(0_usize), Err(MError { kind: MErrorKind::Zero }));
        }

        #[test]
        fn conversion_from_usize_max_64bit() {
            assert_eq!(M::try_from(u32::MAX as usize), Ok(m_some(u32::MAX)));
            // Testing usize::MAX would result in an error, since it's greater than u32::MAX
        }
    }

    // isize tests for 32-bit architecture
    #[cfg(target_pointer_width = "32")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_32bit() {
            assert_eq!(M::try_from(0_isize), Err(MError { kind: MErrorKind::Zero }));
        }

        #[test]
        fn conversion_from_isize_max_32bit() {
            assert_eq!(M::try_from(isize::MAX), Ok(m_some(isize::MAX as u32)));
        }

        #[test]
        fn conversion_from_isize_min_32bit() {
            assert_eq!(M::try_from(isize::MIN), Err(MError { kind: MErrorKind::NegativeValue }));
        }
    }

    // isize tests for 64-bit architecture
    #[cfg(target_pointer_width = "64")]
    mod isize_tests {
        use super::*;

        #[test]
        fn conversion_from_isize_zero_64bit() {
            assert_eq!(M::try_from(0_isize), Err(MError { kind: MErrorKind::Zero }));
        }

        #[test]
        fn conversion_from_isize_max_64bit() {
            let value = isize::MAX.min(u32::MAX as isize) as u32;
            assert_eq!(M::try_from(value), Ok(m_some(value)));
        }

        #[test]
        fn conversion_from_isize_min_64bit() {
            assert_eq!(M::try_from(isize::MIN), Err(MError { kind: MErrorKind::NegativeValue }));
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
        assert_eq!(try_into_m(0u32), Err(MError { kind: MErrorKind::Zero }));
        // Add more tests for other types with value zero
    }

    // Error case: negative value
    #[test]
    fn test_new_negative() {
        assert_eq!(try_into_m(-1i32), Err(MError { kind: MErrorKind::NegativeValue }));
        // Add more tests for other negative i8, i16, i32, i64, isize values
    }

    // Error case: out of range
    #[test]
    fn test_new_out_of_range() {
        assert!(matches!(try_into_m(u64::MAX), Err(MError { kind: MErrorKind::OutOfRange(_), .. })));
        // Add more tests for other types with values out of u32 range
    }

    #[test]
    fn test_new_usize_isize_arch_dependent() {
        let max_usize: usize = u32::MAX as usize;
        assert_eq!(try_into_m(max_usize), Ok(M::Some(core::num::NonZeroU32::new(max_usize as u32).unwrap())));

        let max_usize_plus_one: usize = (u32::MAX as usize).wrapping_add(1);
        assert!(matches!(try_into_m(max_usize_plus_one), Err(MError { kind: MErrorKind::OutOfRange(_), .. })));

        if cfg!(target_pointer_width = "64") {
            let max_isize: isize = u32::MAX as isize;
            assert_eq!(try_into_m(max_isize), Ok(M::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));

            let max_isize_plus_one: isize = (u32::MAX as isize).wrapping_add(1);
            assert!(matches!(try_into_m(max_isize_plus_one), Err(MError { kind: MErrorKind::OutOfRange(_), .. })));

            let min_isize_minus_one: isize = (i32::MIN as isize).wrapping_sub(1);
            assert!(matches!(try_into_m(min_isize_minus_one), Err(MError { kind: MErrorKind::NegativeValue })));
        } else if cfg!(target_pointer_width = "32") {
            // For 32-bit architectures, isize max would be within range
            let max_isize: isize = isize::MAX;
            assert_eq!(try_into_m(max_isize), Ok(M::Some(core::num::NonZeroU32::new(max_isize as u32).unwrap())));
        }
    }
}
