/// Converts a `zno_fhe::Schema` into an `crate::Schema`.
///
impl From<Schema> for zno_helib_sys::Schema {
    /// Converts the given `zno_fhe::Schema` into the corresponding `Schema` variant.
    ///
    /// # Arguments
    ///
    /// * `schema` - The `zno_fhe::Schema` to convert.
    ///
    /// # Returns
    ///
    /// The converted `Schema` variant.
    fn from(schema: Schema) -> Self {
        match schema {
            Schema::None => zno_helib_sys::Schema::default(),
            Schema::Bfv => zno_helib_sys::Schema::Bfv,
            Schema::Ckks => zno_helib_sys::Schema::Ckks,
            Schema::Bgv => zno_helib_sys::Schema::Bgv,
        }
    }
}

/// Converts a `Schema` into an `zno_fhe::Schema`.
///
impl From<zno_helib_sys::Schema> for Schema {
    /// Converts the given `ffi::Schema` into the corresponding `zno_fhe::Schema`.
    ///
    /// # Arguments
    ///
    /// * `schema` - The `ffi::Schema` to convert.
    ///
    /// # Returns
    ///
    /// The converted `zno_fhe::Schema`.
    fn from(schema: zno_helib_sys::Schema) -> Self {
        match schema {
            zno_helib_sys::Schema::None => Schema::default(),
            zno_helib_sys::Schema::Bfv  => Schema::Bfv,
            zno_helib_sys::Schema::Ckks => Schema::Ckks,
            zno_helib_sys::Schema::Bgv  => Schema::Bgv,
        }
    }
}
