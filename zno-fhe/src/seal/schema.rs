
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Schema {
    None = 0x0,
    Bfv = 0x1,
    Ckks = 0x2,
    Bgv = 0x3,
}

/// Converts a `zno_fhe::Schema` into an `crate::Schema`.
///
impl From<super::Schema> for Schema {
    /// Converts the given `zno_fhe::Schema` into the corresponding `Schema` variant.
    ///
    /// # Arguments
    ///
    /// * `schema` - The `zno_fhe::Schema` to convert.
    ///
    /// # Returns
    ///
    /// The converted `Schema` variant.
    fn from(schema: super::Schema) -> Self {
        match schema {
            super::Schema::Bfv => super::Schema::Bfv,
            super::Schema::Bgv => super::Schema::Bgv,
            super::Schema::Ckks => super::Schema::Ckks,
            _ => super::Schema::default(),
        }
    }
}

/// Converts a `Schema` into an `zno_fhe::Schema`.
///
impl From<Schema> for super::Schema {
    /// Converts the given `ffi::Schema` into the corresponding `zno_fhe::Schema`.
    ///
    /// # Arguments
    ///
    /// * `schema` - The `ffi::Schema` to convert.
    ///
    /// # Returns
    ///
    /// The converted `zno_fhe::Schema`.
    fn from(schema: zno_seal_sys::Schema) -> Self {
        match schema {
            Schema::Bfv  => super::Schema::Bfv,
            Schema::Ckks => super::Schema::Ckks,
            Schema::Bgv  => super::Schema::Bgv,
            _ => super::Schema::default(),
        }
    }
}

/// Implements the `Default` trait for `Schema`.
/// Returns the default value for `Schema`, which is `Schema::Bgv`.
impl Default for Schema {
    /// Returns the default value for `Schema`, which is `Schema::Bgv`.
    ///
    /// # Arguments
    ///
    /// * `self` - The `Schema` to set to the default value.
    ///
    /// # Returns
    ///
    /// The default value for `Schema`, which is `Schema::Bgv`.
    fn default() -> Self {
        super::Schema::Bgv
    }
}

/// Converts a `u8` value into a `super::ffi::Schema` enum variant.
///
/// # Arguments
///
/// * `item` - The u8 value to convert.
///
/// # Returns
///
/// The corresponding `super::ffi::Schema` enum variant. The `default()` method is used in two cases:
/// 1. When the u8 value is 0x0. This is a direct mapping to the default variant.
/// 2. When the u8 value does not correspond to any `super::ffi::Schema` variant. In this case, the `default()` method is used to return a default variant (BGV).
///
/// This behavior differs from the C++17 library, where an invalid value would typically result in an error or exception. In this FFI, invalid values are silently converted to the default variant for simplicity and safety.
///
/// # Examples
///
/// ```
/// use super::ffi::*;
///
/// let value: u8 = 0x1;
/// let schema = super::Schema::from(value);
/// assert_eq!(schema, super::Schema::Bfv);
/// ```
impl From<u8> for Schema {
    fn from(item: u8) -> Self {
        match item {
            0x0 => super::Schema::default(),
            0x1 => super::Schema::Bfv,
            0x2 => super::Schema::Ckks,
            0x3 => super::Schema::Bgv,
            _   => super::Schema::default(), // default
        }
    }
}

/// Converts a `Schema` enum variant into a `u8` value.
///
/// # Arguments
///
/// * `item` - The `Schema` enum variant to convert.
///
/// # Returns
///
/// The corresponding `u8` value.
///
/// # Examples
///
/// ```
/// use Schema;
///
/// let schema = super::Schema::Bfv;
/// let u8_value: u8 = schema.into();
/// assert_eq!(u8_value, 0x1); // assuming Bfv corresponds to 0x1
/// ```
impl From<Schema> for u8 {
    fn from(item: Schema) -> Self {
        item as u8
    }
}
