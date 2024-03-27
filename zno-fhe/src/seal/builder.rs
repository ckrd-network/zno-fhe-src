use zno_seal_sys::ffi;

/// This module contains the definition of the `Builder` struct and its associated methods.
/// The `Builder` struct is responsible for constructing a `BGVContextBuilder` object used in the SEAL implementation.
/// It provides methods for setting various parameters and loading data from strings.
/// The `Builder` struct also defines error types for handling null pointer exceptions and other FFI-related errors.
/// The methods of the `Builder` struct are used to configure the parameters of the `BGVContextBuilder` object.
/// The `Builder` struct is used in the process of building a `BGVContextBuilder` object with the desired parameters.
/// Once the `Builder` is configured, the `build()` method can be called to create a `BGVContextBuilder` object.
/// The `Builder` struct is specific to the BGV scheme in the SEAL library.
pub struct Builder {
    // Holds a pointer to the C++ object
    pub inner: cxx::UniquePtr<self::ffi::BGVContextBuilder>,
}

pub trait FheBuilder {
    type P: Parameters;
    type E: FheError;

    fn new(params: Self::P) -> Result<Self, Self::E>
    where
        Self: Sized;
}

/// Define the Rust struct to represent the C++ Context class
pub struct Builder<C: FheBuilder> {
    inner: cxx::UniquePtr<C>,
}

// Define methods for the Rust struct Context.
// Logic common across implementations belongs here.
impl<C: FheBuilder> Context<C> {

    pub fn new<P: Parameters, E: FheError>(params: P) -> Result<Self, E> {
        let inner = C::new(params)?;
        Ok(Self { inner })
    }

    pub fn convert_to_vec(s: &str) -> Vec<i64> {
        s.split(',')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect()
    }

}

impl Setters for Builder {

    // fn set_bits<T, E>(mut self, value: T) -> Result<Self, BGVError>
    // where
    //     Self: Sized,
    //     T: ToU32<E>,
    //     E: Into<SetError>,
    // {
    //     todo!()
    //     // let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
    //     // // Assuming `ffi::set_bits` returns Result<(), BitsError>
    //     // self.inner = ffi::set_bits(self.inner, u32_value);
    //     // Ok(self)
    // }

//     fn set_c<T, E>(mut self, value: T) -> Result<Self, BGVError>
//     where
//         Self: Sized,
//         T: ToU32<E>,
//         E: Into<SetError>,
//     {
//         let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
//         // Assuming `ffi::set_c` returns Result<(), CError>
//         self.inner = ffi::set_c(self.inner, u32_value);
//         Ok(self)
//     }

    fn set_m<T, E>(mut self, value: T) -> Result<Self, BGVError>
    where
        Self: Sized,
        T: ToU32<E>,
        E: Into<SetError>,
    {
        let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
        // Assumes `ffi::set_m` returns Result<(), MError>
        self.inner = ffi::set_m(self.inner, u32_value);
        Ok(self)
    }

//     fn set_p<T, E>(mut self, value: T) -> Result<Self, BGVError>
//     where
//         Self: Sized,
//         T: ToU32<E>,
//         E: Into<SetError>,
//     {
//         let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
//         // Assuming `ffi::set_p` returns Result<(), PError>
//         self.inner = ffi::set_p(self.inner, u32_value);
//         Ok(self)
//     }

//     fn set_r<T, E>(mut self, value: T) -> Result<Self, BGVError>
//     where
//         Self: Sized,
//         T: ToU32<E>,
//         E: Into<SetError>,
//     {
//         let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
//         // Assuming `ffi::set_r` returns Result<(), RError>
//         self.inner = ffi::set_r(self.inner, u32_value);
//         Ok(self)
//     }

    fn set(self, value: Metric) -> Result<Self, BGVError>
    {

        // Convert `value` into `Metric`, since `Into` is infallible
        let metric: Metric = value.into();

        self.metric_set(metric)
    }

    fn try_set<T: TryInto<Metric, Error=BGVError>>(self, value: T) -> Result<Self, BGVError>
    where
        Self: Sized,
    {
        // Convert `value` into `Metric`, since `TryInto` is fallible
        let metric = value.try_into()?;

        self.metric_set(metric)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_with_valid_builder() {
        let builder = Builder::new();
        let context = builder.build();
        assert!(context.is_ok());
    }

    // #[ignore = "Incomplete HELib FFI"]
    // #[test]
    // fn test_bgv_context_new() {
    //     // Set up the input parameters
    //     let context = Context::new(Parameters::default()).expect("BGV context creation should succeed");
    //     let actual_m = context.get_m().expect("Retrieving M value failed"); // Panic if get_m() returns an Err
    //     let expected_m = M::default();
    //     assert_eq!(actual_m, expected_m, "BGV scheme parameter M, should be set correctly");
    // }

    // #[ignore = "Incomplete HELib FFI"]
    // #[test]
    // fn test_get_m_valid_value() {
    //     let context_result = Context::new(Parameters::default()); // Create your context
    //     let m = context_result
    //                 .expect("Expected to successfully retrieve Context")
    //                 .get_m()
    //                 .expect("Expected to successfully retrieve M");
    //     assert_eq!(m, M::default());
    // }

}
