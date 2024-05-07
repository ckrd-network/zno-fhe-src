// Import the necessary types
#[cfg(feature = "seal")]
use zno_seal_sys::bgv::ffi::BGVContextBuilder as FFIBGVBuilder;
use zno_seal_sys::bgv::ffi;

use crate::seal::parameters::*;
use crate::seal::bgv::*;
use crate::seal::bgv::parameters::Parameters;
use crate::seal::setters::*;
use crate::error::*;
use crate::fhe::*;
use crate::FheError;

use crate::prelude::*;

use cxx;

impl crate::seal::builder::FheBuilder for FFIBGVBuilder{
    type P = Parameters;
    type E = BGVError;

    fn new(params: Self::P) -> Result<Self, Self::E> {
        // Call the ffi function to initialize the BGVContextBuilder
        let builder = ffi::init(params.scheme); // Replace `params.scheme` with the actual field or method to get the scheme from `params`

        // Check if the builder is initialized correctly
        if builder.is_null() {
            return Err(Self::E::new("Failed to initialize BGVContextBuilder")); // Replace with the actual method to create a new FheError
        }

        Ok(builder)
    }
}

/// This module contains the definition of the `Builder` struct and its associated methods.
/// The `Builder` struct is responsible for constructing a `BGVContextBuilder` object used in the SEAL implementation.
/// It provides methods for setting various parameters and loading data from strings.
/// The `Builder` struct also defines error types for handling null pointer exceptions and other FFI-related errors.
/// The methods of the `Builder` struct are used to configure the parameters of the `BGVContextBuilder` object.
/// The `Builder` struct is used in the process of building a `BGVContextBuilder` object with the desired parameters.
/// Once the `Builder` is configured, the `build()` method can be called to create a `BGVContextBuilder` object.
/// The `Builder` struct is specific to the BGV scheme in the SEAL library.
/// Define the Rust struct to represent the C++ Builder class
pub struct Builder<B: FheBuilder + cxx::memory::UniquePtrTarget> {
    inner: cxx::UniquePtr<B>,
}

// Define the specialized BGVContextBuilder struct
pub struct BGVBuilder {
    builder: Builder<FFIBGVBuilder>,
}

impl BGVBuilder {
    // Add methods to initialize and configure the builder
    pub fn new<P: FheParameters, E: FheError>(params: P) -> Result<Self, E> {
        let builder = Builder::<FFIBGVBuilder>::new(params)?;
        Ok(Self { builder })
    }

    // Add other methods to configure the builder and build the final object
}
pub trait FheBuilder {
    type P: FheParameters;
    type E: FheError;

    fn new(params: Self::P) -> Result<Self, Self::E>
    where
        Self: Sized;
}


// Define methods for the Rust struct Builder.
// Logic common across implementations belongs here.
impl<B: FheBuilder + cxx::memory::UniquePtrTarget> Builder<B> {

    pub fn new<P: FheParameters, E: FheError>(params: P) -> Result<Self, E> {
        let inner = B::new(params)?;
        Ok(Self { inner })
    }

    pub fn convert_to_vec(s: &str) -> Vec<i64> {
        s.split(',')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect()
    }

}

impl<B: FheBuilder + cxx::memory::UniquePtrTarget> Setters for Builder<B> {

    // fn set_m<T, E>(mut self, value: T) -> Result<Self, BGVError>
    // where
    //     Self: Sized,
    //     T: ToU32<E>,
    //     E: Into<SetError>,
    // {
    //     let u32_value = value.to_u32().map_err(Into::<SetError>::into).map_err(Into::<BGVError>::into)?;
    //     // Assumes `ffi::set_m` returns Result<(), MError>
    //     self.inner = ffi::set_m(self.inner, u32_value);
    //     Ok(self)
    // }

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

    fn set<S, M>(self, value: M) -> Result<Self, BGVError>
    where
        M: FheMetric<S> + Into<M>,
        S: FheScheme,
    {
        // Convert `value` into `Metric`, since `Into` is infallible
        let metric = value.into();

        self.metric_set(metric)
    }

    fn try_set<S, M>(self, value: M) -> Result<Self, BGVError>
    where
        M: FheMetric<S> + TryInto<M, Error=BGVError>,
        S: FheScheme,
        Self: Sized,
    {
        // Convert `value` into `M`, since `TryInto` is fallible
        let metric = value.try_into()?;

        self.metric_set(metric)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::seal::bgv::parameters::Parameters;
    #[test]
    fn test_build_with_valid_builder() {
        let params = Parameters::default();
        let builder = Builder::new(params).unwrap();
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
