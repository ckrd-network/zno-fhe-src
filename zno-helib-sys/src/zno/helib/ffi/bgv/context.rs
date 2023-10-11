#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod prelude {}
use autocxx::prelude::*;
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
// mod ffi {
    pub trait ToCppString {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString>;
    }
    impl ToCppString for &str {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(self)
        }
    }
    impl ToCppString for String {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(&self)
        }
    }
    impl ToCppString for &String {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            make_string(self)
        }
    }
    impl ToCppString for cxx::UniquePtr<cxx::CxxString> {
        fn into_cpp(self) -> cxx::UniquePtr<cxx::CxxString> {
            self
        }
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::Context {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::C,
            ::cxx::o,
            ::cxx::n,
            ::cxx::t,
            ::cxx::e,
            ::cxx::x,
            ::cxx::t,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::NTL::xdouble {
        type Id = (
            ::cxx::N,
            ::cxx::T,
            ::cxx::L,
            (),
            ::cxx::x,
            ::cxx::d,
            ::cxx::o,
            ::cxx::u,
            ::cxx::b,
            ::cxx::l,
            ::cxx::e,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::PowerfulDCRT {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::P,
            ::cxx::o,
            ::cxx::w,
            ::cxx::e,
            ::cxx::r,
            ::cxx::f,
            ::cxx::u,
            ::cxx::l,
            ::cxx::D,
            ::cxx::C,
            ::cxx::R,
            ::cxx::T,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::PolyModRing {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::P,
            ::cxx::o,
            ::cxx::l,
            ::cxx::y,
            ::cxx::M,
            ::cxx::o,
            ::cxx::d,
            ::cxx::R,
            ::cxx::i,
            ::cxx::n,
            ::cxx::g,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::IndexSet {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::I,
            ::cxx::n,
            ::cxx::d,
            ::cxx::e,
            ::cxx::x,
            ::cxx::S,
            ::cxx::e,
            ::cxx::t,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::ThinRecryptData {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::T,
            ::cxx::h,
            ::cxx::i,
            ::cxx::n,
            ::cxx::R,
            ::cxx::e,
            ::cxx::c,
            ::cxx::r,
            ::cxx::y,
            ::cxx::p,
            ::cxx::t,
            ::cxx::D,
            ::cxx::a,
            ::cxx::t,
            ::cxx::a,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::PAlgebra {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::P,
            ::cxx::A,
            ::cxx::l,
            ::cxx::g,
            ::cxx::e,
            ::cxx::b,
            ::cxx::r,
            ::cxx::a,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::PAlgebraMod {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::P,
            ::cxx::A,
            ::cxx::l,
            ::cxx::g,
            ::cxx::e,
            ::cxx::b,
            ::cxx::r,
            ::cxx::a,
            ::cxx::M,
            ::cxx::o,
            ::cxx::d,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::EncryptedArray {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::E,
            ::cxx::n,
            ::cxx::c,
            ::cxx::r,
            ::cxx::y,
            ::cxx::p,
            ::cxx::t,
            ::cxx::e,
            ::cxx::d,
            ::cxx::A,
            ::cxx::r,
            ::cxx::r,
            ::cxx::a,
            ::cxx::y,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::ModuliSizes {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::M,
            ::cxx::o,
            ::cxx::d,
            ::cxx::u,
            ::cxx::l,
            ::cxx::i,
            ::cxx::S,
            ::cxx::i,
            ::cxx::z,
            ::cxx::e,
            ::cxx::s,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::Cmodulus {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::C,
            ::cxx::m,
            ::cxx::o,
            ::cxx::d,
            ::cxx::u,
            ::cxx::l,
            ::cxx::u,
            ::cxx::s,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::NTL::ZZ {
        type Id = (::cxx::N, ::cxx::T, ::cxx::L, (), ::cxx::Z, ::cxx::Z);
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::JsonWrapper {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::J,
            ::cxx::s,
            ::cxx::o,
            ::cxx::n,
            ::cxx::W,
            ::cxx::r,
            ::cxx::a,
            ::cxx::p,
            ::cxx::p,
            ::cxx::e,
            ::cxx::r,
        );
        type Kind = cxx::kind::Opaque;
    }
    mod bindgen {
        pub(super) mod root {
            pub use cxxbridge::NTL_Vec_long_AutocxxConcrete;
            pub use cxxbridge::std_basic_ostream_char_AutocxxConcrete;
            pub use cxxbridge::std_basic_istream_char_AutocxxConcrete;
            pub mod NTL {
                #[repr(C, align(8))]
                pub struct xdouble {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 16],
                }
                #[repr(C, align(8))]
                pub struct ZZ {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 8],
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::NTL::xdouble {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::NTL::xdouble {
                        cxxbridge::xdouble_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::NTL::xdouble) {
                        cxxbridge::xdouble_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::new::CopyNew for root::NTL::xdouble {
                    ///Synthesized copy constructor.
                    unsafe fn copy_new(
                        other: &root::NTL::xdouble,
                        this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<root::NTL::xdouble>>,
                    ) {
                        cxxbridge :: NTL_xdouble_new_synthetic_const_copy_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464 (this . get_unchecked_mut () . as_mut_ptr () , other)
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::NTL::ZZ {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::NTL::ZZ {
                        cxxbridge::ZZ_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::NTL::ZZ) {
                        cxxbridge::ZZ_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::new::CopyNew for root::NTL::ZZ {
                    unsafe fn copy_new(
                        a: &root::NTL::ZZ,
                        this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<root::NTL::ZZ>>,
                    ) {
                        cxxbridge::NTL_ZZ_new3_autocxx_wrapper_0xe97f7296bea4f464(
                            this.get_unchecked_mut().as_mut_ptr(),
                            a,
                        )
                    }
                }
                unsafe impl autocxx::moveit::new::MoveNew for root::NTL::ZZ {
                    unsafe fn move_new(
                        mut a: ::core::pin::Pin<autocxx::moveit::MoveRef<'_, root::NTL::ZZ>>,
                        this: ::core::pin::Pin<&mut ::core::mem::MaybeUninit<root::NTL::ZZ>>,
                    ) {
                        cxxbridge::new11_autocxx_wrapper_0xe97f7296bea4f464(
                            this.get_unchecked_mut().as_mut_ptr(),
                            {
                                let r: &mut _ = ::core::pin::Pin::into_inner_unchecked(a.as_mut());
                                r
                            },
                        )
                    }
                }
                #[allow(unused_imports)]
                use self::super::super::super::{cxxbridge, ToCppString};
                #[allow(unused_imports)]
                use self::super::super::root;
            }
            pub mod helib {
                /// @class Context
                /// @brief Maintaining the HE scheme parameters
                #[repr(C, align(8))]
                pub struct Context {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 840],
                }
                /// @class PowerfulDCRT
                /// @brief Conversion between powerful representation, DoubleCRT, and ZZX
                #[repr(C, align(8))]
                pub struct PowerfulDCRT {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 168],
                }
                /// @struct PolyModRing
                /// @brief Lightweight type for describing the structure of a single slot of the
                /// plaintext space.
                ///
                /// A single slot of the plaintext space is isomorphic to
                /// \f$\mathbb{Z}[X]/(G(x),p^r)\f$ for some irreducible factor G of
                /// \f$\Phi_m(X)\f$, so the main useful members of this `struct` are `p`, `r`,
                /// `G`, and `p2r`.
                ///
                /// The fields of this `struct` are all `const`, so they should be determined
                /// at the time of construction.
                ///
                /// @note This `struct` aggregates this often-useful information into a single
                /// placeholder for convenience.
                #[repr(C, align(8))]
                pub struct PolyModRing {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 32],
                }
                ///! @brief A dynamic set of non-negative integers.
                ///!
                ///! You can iterate through a set as follows:
                ///! \code
                ///!    for (long i = s.first(); i <= s.last(); i = s.next(i)) ...
                ///!    for (long i = s.last(); i >= s.first(); i = s.prev(i)) ...
                ///! \endcode
                #[repr(C, align(8))]
                pub struct IndexSet {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 64],
                }
                ///! @class ThinRecryptData
                ///! @brief Same as above, but for "thin" bootstrapping, where the slots
                ///! are assumed to contain constants
                #[repr(C, align(8))]
                pub struct ThinRecryptData {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 176],
                }
                /// @class PAlgebra
                /// @brief The structure of (Z/mZ)* /(p)
                ///
                /// A PAlgebra object is determined by an integer m and a prime p, where p does
                /// not divide m. It holds information describing the structure of (Z/mZ)^*,
                /// which is isomorphic to the Galois group over A = Z[X]/Phi_m(X)).
                ///
                /// We represent (Z/mZ)^* as (Z/mZ)^* = (p) x (g1,g2,...) x (h1,h2,...)
                /// where the group generated by g1,g2,... consists of the elements that
                /// have the same order in (Z/mZ)^* as in (Z/mZ)^* /(p,g_1,...,g_{i-1}), and
                /// h1,h2,... generate the remaining quotient group (Z/mZ)^* /(p,g1,g2,...).
                ///
                /// We let T subset (Z/mZ)^* be a set of representatives for the quotient
                /// group (Z/mZ)^* /(p), defined as T={ prod_i gi^{ei} * prod_j hj^{ej} }
                /// where the ei's range over 0,1,...,ord(gi)-1 and the ej's range over
                /// 0,1,...ord(hj)-1 (these last orders are in (Z/mZ)^* /(p,g1,g2,...)).
                ///
                /// Phi_m(X) is factored as Phi_m(X)= prod_{t in T} F_t(X) mod p,
                /// where the F_t's are irreducible modulo p. An arbitrary factor
                /// is chosen as F_1, then for each t in T we associate with the index t the
                /// factor F_t(X) = GCD(F_1(X^t), Phi_m(X)).
                ///
                /// Note that fixing a representation of the field R=(Z/pZ)[X]/F_1(X)
                /// and letting z be a root of F_1 in R (which
                /// is a primitive m-th root of unity in R), we get that F_t is the minimal
                /// polynomial of z^{1/t}.
                #[repr(C, align(8))]
                pub struct PAlgebra {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 288],
                }
                #[repr(C, align(8))]
                pub struct PAlgebraMod {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 8],
                }
                ///! @class EncryptedArray
                ///! @brief A simple wrapper for a smart pointer to an EncryptedArrayBase.
                ///! This is the interface that higher-level code should use
                #[repr(C, align(8))]
                pub struct EncryptedArray {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 16],
                }
                ///! A helper class to map required modulo-sizes to primeSets
                #[repr(C, align(8))]
                pub struct ModuliSizes {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 32],
                }
                /// @class Cmodulus
                /// @brief Provides FFT and iFFT routines modulo a single-precision prime
                ///
                /// On initialization, it initializes NTL's zz_pContext for this q
                /// and computes a 2m-th root of unity r mod q and also r^{-1} mod q.
                /// Thereafter this class provides FFT and iFFT routines that converts between
                /// time & frequency domains. Some tables are computed the first time that
                /// each directions is called, which are then used in subsequent computations.
                ///
                /// The "time domain" polynomials are represented as ZZX, which are reduced
                /// modulo Phi_m(X). The "frequency domain" are just vectors of integers
                /// (vec_long), that store only the evaluation in primitive m-th
                /// roots of unity.
                #[repr(C, align(8))]
                pub struct Cmodulus {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 128],
                }
                #[repr(C, align(8))]
                pub struct JsonWrapper {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 16],
                }
                impl Context {
                    ///autocxx bindings couldn't be generated: This method is private
                    fn readParamsFrom(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This method is private
                    fn readParamsFromJSON(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This method is private
                    fn addSpecialPrimes(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This method is private
                    fn addCtxtPrimes(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This method is private
                    fn addSmallPrimes(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This method is private
                    fn addSmallPrime(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This method is private
                    fn addCtxtPrime(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This method is private
                    fn addSpecialPrime(_uhoh: autocxx::BindingGenerationFailure) {}
                    /// @brief Getter method for the `m` used to create this `context`.
                    /// @return The cyclotomic index `m`.
                    pub fn getM(self: &root::helib::Context) -> autocxx::c_long {
                        cxxbridge::helib_Context_getM_autocxx_wrapper_0xe97f7296bea4f464(self)
                    }
                    /// @brief Getter method for the `p` used to create this `context`.
                    /// @return The plaintext modulus `p`.
                    pub fn getP(self: &root::helib::Context) -> autocxx::c_long {
                        cxxbridge::helib_Context_getP_autocxx_wrapper_0xe97f7296bea4f464(self)
                    }
                    /// @brief Getter method for the `phi(m)` of the created `context`.
                    /// @return The degree of the cyclotomic polynomial `Phi_m(X)`.
                    pub fn getPhiM(self: &root::helib::Context) -> autocxx::c_long {
                        cxxbridge::helib_Context_getPhiM_autocxx_wrapper_0xe97f7296bea4f464(self)
                    }
                    /// @brief Getter method for the `ord(p)` of the created `context`.
                    /// @return The order of `p` in `(Z/mZ)^*`.
                    pub fn getOrdP(self: &root::helib::Context) -> autocxx::c_long {
                        cxxbridge::helib_Context_getOrdP_autocxx_wrapper_0xe97f7296bea4f464(self)
                    }
                    /// @brief Getter method for the number of plaintext slots of the created
                    /// `context`.
                    /// @return The number of plaintext slots `phi(m)/ord(p)`.
                    pub fn getNSlots(self: &root::helib::Context) -> autocxx::c_long {
                        cxxbridge::helib_Context_getNSlots_autocxx_wrapper_0xe97f7296bea4f464(self)
                    }
                    /// @brief Getter method for the standard deviation used..
                    /// @return the standard deviation as an `NTL::xdouble`.
                    pub fn getStdev<'a>(
                        self: &'a root::helib::Context,
                    ) -> impl autocxx::moveit::new::New<Output = root::NTL::xdouble> + 'a
                    {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |placement_return_type| {
                                let placement_return_type =
                                    placement_return_type.get_unchecked_mut().as_mut_ptr();
                                cxxbridge::getStdev_autocxx_wrapper_0xe97f7296bea4f464(
                                    self,
                                    placement_return_type,
                                )
                            })
                        }
                    }
                    /// @brief Getter method for the default `r` value of the created `context`.
                    /// @return The `r` value representing the Hensel lifting for `BGV` or the bit
                    /// precision for `CKKS`.
                    /// @note This value is not invariant: it is possible to work "view" objects
                    /// that use different `PAlgebra` objects.
                    pub fn getR(self: &root::helib::Context) -> autocxx::c_long {
                        cxxbridge::helib_Context_getR_autocxx_wrapper_0xe97f7296bea4f464(self)
                    }
                    /// @brief Getter method for the default `p^r` value of the created `context`.
                    /// @return The raised plaintext modulus `p^r`.
                    /// @note This value is not invariant: it is possible to work "view" objects
                    /// that use different `PAlgebra` objects.
                    pub fn getPPowR(self: &root::helib::Context) -> autocxx::c_long {
                        cxxbridge::helib_Context_getPPowR_autocxx_wrapper_0xe97f7296bea4f464(self)
                    }
                    /// @brief Get the underlying `zMStar` object.
                    /// @return A `zMStar` object.
                    pub fn getZMStar(self: &root::helib::Context) -> &root::helib::PAlgebra {
                        cxxbridge::helib_Context_getZMStar_autocxx_wrapper_0xe97f7296bea4f464(self)
                    }
                    pub fn noiseBoundForUniform1<'a>(
                        self: &'a root::helib::Context,
                        magBound: impl autocxx::ValueParam<root::NTL::xdouble> + 'a,
                        degBound: autocxx::c_long,
                    ) -> impl autocxx::moveit::new::New<Output = root::NTL::xdouble> + 'a
                    {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |placement_return_type| {
                                let mut space0 = autocxx::ValueParamHandler::default();
                                let mut space0 = ::core::pin::Pin::new_unchecked(&mut space0);
                                space0.as_mut().populate(magBound);
                                let placement_return_type =
                                    placement_return_type.get_unchecked_mut().as_mut_ptr();
                                cxxbridge::noiseBoundForUniform1_autocxx_wrapper_0xe97f7296bea4f464(
                                    self,
                                    space0.get_ptr(),
                                    degBound,
                                    placement_return_type,
                                )
                            })
                        }
                    }
                    pub fn noiseBoundForGaussian1<'a>(
                        self: &'a root::helib::Context,
                        sigma: impl autocxx::ValueParam<root::NTL::xdouble> + 'a,
                        degBound: autocxx::c_long,
                    ) -> impl autocxx::moveit::new::New<Output = root::NTL::xdouble> + 'a
                    {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |placement_return_type| {
                                let mut space0 = autocxx::ValueParamHandler::default();
                                let mut space0 = ::core::pin::Pin::new_unchecked(&mut space0);
                                space0.as_mut().populate(sigma);
                                let placement_return_type =
                                    placement_return_type.get_unchecked_mut().as_mut_ptr();
                                cxxbridge::noiseBoundForGaussian1_autocxx_wrapper_0xe97f7296bea4f464(
                                    self,
                                    space0.get_ptr(),
                                    degBound,
                                    placement_return_type,
                                )
                            })
                        }
                    }
                    ///autocxx bindings couldn't be generated: Function operator_equals has a reference return value, but >1 input reference parameters, so the lifetime of the output reference cannot be deduced.
                    fn operator_equals(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: autocxx does not know how to generate bindings to operator=
                    fn operator_equals1(_uhoh: autocxx::BindingGenerationFailure) {}
                    /// @brief Getter method that returns the handles of both the `ctxtPrimes` and
                    /// `specialPrimes` associated with this `Context`.
                    /// @return `IndexSet` of the handles to the `ctxtPrimes` and `specialPrimes`.
                    pub fn fullPrimes<'a>(
                        self: &'a root::helib::Context,
                    ) -> impl autocxx::moveit::new::New<Output = root::helib::IndexSet> + 'a
                    {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |placement_return_type| {
                                let placement_return_type =
                                    placement_return_type.get_unchecked_mut().as_mut_ptr();
                                cxxbridge::fullPrimes_autocxx_wrapper_0xe97f7296bea4f464(
                                    self,
                                    placement_return_type,
                                )
                            })
                        }
                    }
                    /// @brief Getter method that returns the handles of all primes associated with
                    /// this `Context`.
                    /// @return `IndexSet` of handles to the `ctxtPrimes`, `specialPrimes` and
                    /// `smallPrimes`.
                    pub fn allPrimes<'a>(
                        self: &'a root::helib::Context,
                    ) -> impl autocxx::moveit::new::New<Output = root::helib::IndexSet> + 'a
                    {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |placement_return_type| {
                                let placement_return_type =
                                    placement_return_type.get_unchecked_mut().as_mut_ptr();
                                cxxbridge::allPrimes_autocxx_wrapper_0xe97f7296bea4f464(
                                    self,
                                    placement_return_type,
                                )
                            })
                        }
                    }
                    /// @brief Getter method that returns the first `nprimes` `ctxtPrimes`
                    /// associated with this `Context`.
                    /// @param nprimes The number of desired `ctxtPrimes`.
                    /// @return `IndexSet` of handles to the first `nprimes` `ctxtPrimes`.
                    pub fn getCtxtPrimes1<'a>(
                        self: &'a root::helib::Context,
                        nprimes: autocxx::c_long,
                    ) -> impl autocxx::moveit::new::New<Output = root::helib::IndexSet> + 'a
                    {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |placement_return_type| {
                                let placement_return_type =
                                    placement_return_type.get_unchecked_mut().as_mut_ptr();
                                cxxbridge::getCtxtPrimes1_autocxx_wrapper_0xe97f7296bea4f464(
                                    self,
                                    nprimes,
                                    placement_return_type,
                                )
                            })
                        }
                    }
                    pub fn productOfPrimes1<'a>(
                        self: &'a root::helib::Context,
                        s: &'a root::helib::IndexSet,
                    ) -> impl autocxx::moveit::new::New<Output = root::NTL::ZZ> + 'a
                    {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |placement_return_type| {
                                let placement_return_type =
                                    placement_return_type.get_unchecked_mut().as_mut_ptr();
                                cxxbridge::productOfPrimes1_autocxx_wrapper_0xe97f7296bea4f464(
                                    self,
                                    s,
                                    placement_return_type,
                                )
                            })
                        }
                    }
                    /// @brief Print out algebra and other important info
                    /// @param out Output `std::ostream`.
                    pub fn printout(
                        self: &root::helib::Context,
                        out: ::core::pin::Pin<&mut root::std_basic_ostream_char_AutocxxConcrete>,
                    ) {
                        cxxbridge::helib_Context_printout_autocxx_wrapper_0xe97f7296bea4f464(
                            self, out,
                        )
                    }
                    /// @brief Write out the `Context` object in binary format.
                    /// @param str Output `std::ostream`.
                    pub fn writeTo(
                        self: &root::helib::Context,
                        str_: ::core::pin::Pin<&mut root::std_basic_ostream_char_AutocxxConcrete>,
                    ) {
                        cxxbridge::helib_Context_writeTo_autocxx_wrapper_0xe97f7296bea4f464(
                            self, str_,
                        )
                    }
                    /// @brief Read from the stream the serialized `Context` object in binary
                    /// format.
                    /// @param str Input `std::istream`.
                    /// @return The deserialized `Context` object.
                    pub fn readFrom<'a>(
                        str_: ::core::pin::Pin<
                            &'a mut root::std_basic_istream_char_AutocxxConcrete,
                        >,
                    ) -> impl autocxx::moveit::new::New<Output = root::helib::Context> + 'a
                    {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |placement_return_type| {
                                let placement_return_type =
                                    placement_return_type.get_unchecked_mut().as_mut_ptr();
                                cxxbridge::helib_Context_readFrom_autocxx_wrapper_0xe97f7296bea4f464(
                                    str_,
                                    placement_return_type,
                                )
                            })
                        }
                    }
                    /// @brief Read from the stream the serialized `Context` object in binary
                    /// format.
                    /// @param str Input `std::istream`.
                    /// @return Raw pointer to the deserialized `Context` object.
                    pub fn readPtrFrom(
                        str_: ::core::pin::Pin<&mut root::std_basic_istream_char_AutocxxConcrete>,
                    ) -> *mut root::helib::Context {
                        cxxbridge::readPtrFrom_autocxx_wrapper_0xe97f7296bea4f464(str_)
                    }
                    /// @brief Write out the `Context` object to the output stream using JSON
                    /// format.
                    /// @param str Output `std::ostream`.
                    pub fn writeToJSON(
                        self: &root::helib::Context,
                        str_: ::core::pin::Pin<&mut root::std_basic_ostream_char_AutocxxConcrete>,
                    ) {
                        cxxbridge::helib_Context_writeToJSON_autocxx_wrapper_0xe97f7296bea4f464(
                            self, str_,
                        )
                    }
                    /// @brief Write out the `Context` object to a `JsonWrapper`.
                    /// @return The `JsonWrapper`.
                    pub fn writeToJSON1<'a>(
                        self: &'a root::helib::Context,
                    ) -> impl autocxx::moveit::new::New<Output = root::helib::JsonWrapper> + 'a
                    {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |placement_return_type| {
                                let placement_return_type =
                                    placement_return_type.get_unchecked_mut().as_mut_ptr();
                                cxxbridge :: helib_Context_writeToJSON1_autocxx_wrapper_0xe97f7296bea4f464 (self , placement_return_type)
                            })
                        }
                    }
                    /// @brief Read from the stream the serialized `Context` object using JSON
                    /// format.
                    /// @param str Input `std::istream`.
                    /// @return The deserialized `Context` object.
                    pub fn readFromJSON<'a>(
                        str_: ::core::pin::Pin<
                            &'a mut root::std_basic_istream_char_AutocxxConcrete,
                        >,
                    ) -> impl autocxx::moveit::new::New<Output = root::helib::Context> + 'a
                    {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |placement_return_type| {
                                let placement_return_type =
                                    placement_return_type.get_unchecked_mut().as_mut_ptr();
                                cxxbridge :: helib_Context_readFromJSON_autocxx_wrapper_0xe97f7296bea4f464 (str_ , placement_return_type)
                            })
                        }
                    }
                    /// @brief Read from the `JsonWrapper` the serialized `Context` object.
                    /// @param j The `JsonWrapper` containing the serialized `Context` object.
                    /// @return The deserialized `Context` object.
                    pub fn readFromJSON1<'a>(
                        j: &'a root::helib::JsonWrapper,
                    ) -> impl autocxx::moveit::new::New<Output = root::helib::Context> + 'a
                    {
                        unsafe {
                            autocxx::moveit::new::by_raw(move |placement_return_type| {
                                let placement_return_type =
                                    placement_return_type.get_unchecked_mut().as_mut_ptr();
                                cxxbridge :: helib_Context_readFromJSON1_autocxx_wrapper_0xe97f7296bea4f464 (j , placement_return_type)
                            })
                        }
                    }
                    /// @brief Read from the `JsonWrapper` the serialized `Context` object.
                    /// @param j The `JsonWrapper` containing the serialized `Context` object.
                    /// @return Raw pointer to the deserialized `Context` object.
                    pub fn readPtrFromJSON(
                        str_: ::core::pin::Pin<&mut root::std_basic_istream_char_AutocxxConcrete>,
                    ) -> *mut root::helib::Context {
                        cxxbridge::readPtrFromJSON_autocxx_wrapper_0xe97f7296bea4f464(str_)
                    }
                    ///autocxx bindings couldn't be generated: Problem handling function argument gens: A C++ std::vector was found containing some type that cxx can't accommodate as a vector element (long)
                    fn new(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: Problem handling function argument gens: A C++ std::vector was found containing some type that cxx can't accommodate as a vector element (long)
                    fn new1(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This method is private
                    fn new2(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This function was marked =delete
                    fn new3(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This function was marked =delete
                    fn new4(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This function was marked =delete
                    fn new5(_uhoh: autocxx::BindingGenerationFailure) {}
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::Context {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::helib::Context {
                        cxxbridge::Context_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::helib::Context) {
                        cxxbridge::Context_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                impl Drop for root::helib::Context {
                    /// @brief Default destructor.
                    fn drop(self: &mut root::helib::Context) {
                        unsafe {
                            cxxbridge::Context_destructor_autocxx_wrapper_0xe97f7296bea4f464(self)
                        }
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::PowerfulDCRT {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::helib::PowerfulDCRT
                    {
                        cxxbridge::PowerfulDCRT_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::helib::PowerfulDCRT) {
                        cxxbridge::PowerfulDCRT_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::PolyModRing {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::helib::PolyModRing
                    {
                        cxxbridge::PolyModRing_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::helib::PolyModRing) {
                        cxxbridge::PolyModRing_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::new::CopyNew for root::helib::PolyModRing {
                    /// @brief Copy constructor.
                    unsafe fn copy_new(
                        other: &root::helib::PolyModRing,
                        this: ::core::pin::Pin<
                            &mut ::core::mem::MaybeUninit<root::helib::PolyModRing>,
                        >,
                    ) {
                        cxxbridge::helib_PolyModRing_new1_autocxx_wrapper_0xe97f7296bea4f464(
                            this.get_unchecked_mut().as_mut_ptr(),
                            other,
                        )
                    }
                }
                unsafe impl autocxx::moveit::new::MoveNew for root::helib::PolyModRing {
                    /// @brief Move constructor.
                    unsafe fn move_new(
                        mut other: ::core::pin::Pin<
                            autocxx::moveit::MoveRef<'_, root::helib::PolyModRing>,
                        >,
                        this: ::core::pin::Pin<
                            &mut ::core::mem::MaybeUninit<root::helib::PolyModRing>,
                        >,
                    ) {
                        cxxbridge::helib_PolyModRing_new2_autocxx_wrapper_0xe97f7296bea4f464(
                            this.get_unchecked_mut().as_mut_ptr(),
                            {
                                let r: &mut _ =
                                    ::core::pin::Pin::into_inner_unchecked(other.as_mut());
                                r
                            },
                        )
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::IndexSet {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::helib::IndexSet {
                        cxxbridge::IndexSet_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::helib::IndexSet) {
                        cxxbridge::IndexSet_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::new::MoveNew for root::helib::IndexSet {
                    ///Synthesized move constructor.
                    unsafe fn move_new(
                        mut other: ::core::pin::Pin<
                            autocxx::moveit::MoveRef<'_, root::helib::IndexSet>,
                        >,
                        this: ::core::pin::Pin<
                            &mut ::core::mem::MaybeUninit<root::helib::IndexSet>,
                        >,
                    ) {
                        cxxbridge :: helib_IndexSet_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464 (this . get_unchecked_mut () . as_mut_ptr () , { let r : & mut _ = :: core :: pin :: Pin :: into_inner_unchecked (other . as_mut ()) ; r })
                    }
                }
                unsafe impl autocxx::moveit::new::CopyNew for root::helib::IndexSet {
                    ///Synthesized copy constructor.
                    unsafe fn copy_new(
                        other: &root::helib::IndexSet,
                        this: ::core::pin::Pin<
                            &mut ::core::mem::MaybeUninit<root::helib::IndexSet>,
                        >,
                    ) {
                        cxxbridge :: helib_IndexSet_new_synthetic_const_copy_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464 (this . get_unchecked_mut () . as_mut_ptr () , other)
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::ThinRecryptData {
                    unsafe fn allocate_uninitialized_cpp_storage(
                    ) -> *mut root::helib::ThinRecryptData {
                        cxxbridge::ThinRecryptData_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(
                        arg0: *mut root::helib::ThinRecryptData,
                    ) {
                        cxxbridge::ThinRecryptData_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::PAlgebra {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::helib::PAlgebra {
                        cxxbridge::PAlgebra_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::helib::PAlgebra) {
                        cxxbridge::PAlgebra_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::PAlgebraMod {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::helib::PAlgebraMod
                    {
                        cxxbridge::PAlgebraMod_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::helib::PAlgebraMod) {
                        cxxbridge::PAlgebraMod_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::EncryptedArray {
                    unsafe fn allocate_uninitialized_cpp_storage(
                    ) -> *mut root::helib::EncryptedArray {
                        cxxbridge::EncryptedArray_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(
                        arg0: *mut root::helib::EncryptedArray,
                    ) {
                        cxxbridge::EncryptedArray_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::ModuliSizes {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::helib::ModuliSizes
                    {
                        cxxbridge::ModuliSizes_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::helib::ModuliSizes) {
                        cxxbridge::ModuliSizes_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::new::MoveNew for root::helib::ModuliSizes {
                    ///Synthesized move constructor.
                    unsafe fn move_new(
                        mut other: ::core::pin::Pin<
                            autocxx::moveit::MoveRef<'_, root::helib::ModuliSizes>,
                        >,
                        this: ::core::pin::Pin<
                            &mut ::core::mem::MaybeUninit<root::helib::ModuliSizes>,
                        >,
                    ) {
                        cxxbridge :: helib_ModuliSizes_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464 (this . get_unchecked_mut () . as_mut_ptr () , { let r : & mut _ = :: core :: pin :: Pin :: into_inner_unchecked (other . as_mut ()) ; r })
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::Cmodulus {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::helib::Cmodulus {
                        cxxbridge::Cmodulus_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::helib::Cmodulus) {
                        cxxbridge::Cmodulus_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::new::CopyNew for root::helib::Cmodulus {
                    ///! Copy constructor
                    unsafe fn copy_new(
                        other: &root::helib::Cmodulus,
                        this: ::core::pin::Pin<
                            &mut ::core::mem::MaybeUninit<root::helib::Cmodulus>,
                        >,
                    ) {
                        cxxbridge::helib_Cmodulus_new2_autocxx_wrapper_0xe97f7296bea4f464(
                            this.get_unchecked_mut().as_mut_ptr(),
                            other,
                        )
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::JsonWrapper {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::helib::JsonWrapper
                    {
                        cxxbridge::JsonWrapper_alloc_autocxx_wrapper_0xe97f7296bea4f464()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::helib::JsonWrapper) {
                        cxxbridge::JsonWrapper_free_autocxx_wrapper_0xe97f7296bea4f464(arg0)
                    }
                }
                unsafe impl autocxx::moveit::new::MoveNew for root::helib::JsonWrapper {
                    ///Synthesized move constructor.
                    unsafe fn move_new(
                        mut other: ::core::pin::Pin<
                            autocxx::moveit::MoveRef<'_, root::helib::JsonWrapper>,
                        >,
                        this: ::core::pin::Pin<
                            &mut ::core::mem::MaybeUninit<root::helib::JsonWrapper>,
                        >,
                    ) {
                        cxxbridge :: helib_JsonWrapper_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464 (this . get_unchecked_mut () . as_mut_ptr () , { let r : & mut _ = :: core :: pin :: Pin :: into_inner_unchecked (other . as_mut ()) ; r })
                    }
                }
                #[allow(unused_imports)]
                use self::super::super::super::{cxxbridge, ToCppString};
                #[allow(unused_imports)]
                use self::super::super::root;
            }
            pub mod std {
                pub type ostream = root::std_basic_ostream_char_AutocxxConcrete;
                pub type istream = root::std_basic_istream_char_AutocxxConcrete;
                #[allow(unused_imports)]
                use self::super::super::super::{cxxbridge, ToCppString};
                #[allow(unused_imports)]
                use self::super::super::root;
            }
            #[allow(unused_imports)]
            use self::super::super::{cxxbridge, ToCppString};
            #[allow(unused_imports)]
            use self::super::root;
        }
    }
    #[deny(improper_ctypes, improper_ctypes_definitions)]
    #[allow(clippy::unknown_clippy_lints)]
    #[allow(
        non_camel_case_types,
        non_snake_case,
        clippy::extra_unused_type_parameters,
        clippy::items_after_statements,
        clippy::ptr_as_ptr,
        clippy::upper_case_acronyms,
        clippy::use_self
    )]
    mod cxxbridge {
        pub fn autocxx_make_string_0xe97f7296bea4f464(
            str_: &str,
        ) -> ::cxx::UniquePtr<::cxx::CxxString> {
            extern "C" {
                #[link_name = "cxxbridge1$autocxx_make_string_0xe97f7296bea4f464"]
                fn __autocxx_make_string_0xe97f7296bea4f464(
                    str_: ::cxx::private::RustStr,
                ) -> *mut ::cxx::CxxString;
            }
            unsafe {
                ::cxx::UniquePtr::from_raw(__autocxx_make_string_0xe97f7296bea4f464(
                    ::cxx::private::RustStr::from(str_),
                ))
            }
        }
        pub unsafe fn Context_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut Context {
            extern "C" {
                #[link_name = "cxxbridge1$Context_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __Context_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __Context_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn Context_free_autocxx_wrapper_0xe97f7296bea4f464(arg0: *mut Context) {
            extern "C" {
                #[link_name = "cxxbridge1$Context_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __Context_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __Context_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        /// @class Context
        /// @brief Maintaining the HE scheme parameters
        pub type Context = super::bindgen::root::helib::Context;
        /// @brief Getter method for the `m` used to create this `context`.
        /// @return The cyclotomic index `m`.
        pub fn helib_Context_getM_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_getM_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_getM_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __helib_Context_getM_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        /// @brief Getter method for the `p` used to create this `context`.
        /// @return The plaintext modulus `p`.
        pub fn helib_Context_getP_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_getP_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_getP_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __helib_Context_getP_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        /// @brief Getter method for the `phi(m)` of the created `context`.
        /// @return The degree of the cyclotomic polynomial `Phi_m(X)`.
        pub fn helib_Context_getPhiM_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_getPhiM_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_getPhiM_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __helib_Context_getPhiM_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        /// @brief Getter method for the `ord(p)` of the created `context`.
        /// @return The order of `p` in `(Z/mZ)^*`.
        pub fn helib_Context_getOrdP_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_getOrdP_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_getOrdP_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __helib_Context_getOrdP_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        /// @brief Getter method for the number of plaintext slots of the created
        /// `context`.
        /// @return The number of plaintext slots `phi(m)/ord(p)`.
        pub fn helib_Context_getNSlots_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_getNSlots_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_getNSlots_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __helib_Context_getNSlots_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        impl Context {
            /// @brief Getter method for the scale.
            /// @return the scale as a `double`.
            pub fn getScale(&self) -> f64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getScale"]
                    fn __getScale(_: &Context) -> f64;
                }
                unsafe { __getScale(self) }
            }
        }
        /// @brief Getter method for the standard deviation used..
        /// @return the standard deviation as an `NTL::xdouble`.
        pub unsafe fn getStdev_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
            placement_return_type: *mut xdouble,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$getStdev_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __getStdev_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    placement_return_type: *mut ::cxx::core::ffi::c_void,
                );
            }
            __getStdev_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                placement_return_type.cast(),
            )
        }
        /// @brief Getter method for the default `r` value of the created `context`.
        /// @return The `r` value representing the Hensel lifting for `BGV` or the bit
        /// precision for `CKKS`.
        /// @note This value is not invariant: it is possible to work "view" objects
        /// that use different `PAlgebra` objects.
        pub fn helib_Context_getR_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_getR_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_getR_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __helib_Context_getR_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        /// @brief Getter method for the default `p^r` value of the created `context`.
        /// @return The raised plaintext modulus `p^r`.
        /// @note This value is not invariant: it is possible to work "view" objects
        /// that use different `PAlgebra` objects.
        pub fn helib_Context_getPPowR_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
        ) -> c_long {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_getPPowR_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_getPPowR_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    __return: *mut c_long,
                );
            }
            unsafe {
                let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                __helib_Context_getPPowR_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                    __return.as_mut_ptr(),
                );
                __return.assume_init()
            }
        }
        impl Context {
            /// @brief Getter method for the `precision` value of the created
            /// `CKKS` `context`.
            /// @return The bit `precision` value.
            /// @note This value is not invariant: it is possible to work "view" objects
            /// that use different `PAlgebra` objects.
            pub fn getPrecision(&self) -> c_long {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getPrecision"]
                    fn __getPrecision(_: &Context, __return: *mut c_long);
                }
                unsafe {
                    let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                    __getPrecision(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        impl Context {
            /// @brief Get a powerful converter.
            /// @return A powerful converter.
            pub fn getPowerfulConverter(&self) -> &PowerfulDCRT {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getPowerfulConverter"]
                    fn __getPowerfulConverter(_: &Context) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { &*__getPowerfulConverter(self).cast() }
            }
        }
        impl<'a> Context {
            /// @brief Get a slot ring.
            /// @return A reference to a `std::shared` pointer pointing to a slotRing.
            pub fn getSlotRing(&'a self) -> &'a ::cxx::SharedPtr<PolyModRing> {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getSlotRing"]
                    fn __getSlotRing<'a>(_: &'a Context) -> &'a ::cxx::SharedPtr<PolyModRing>;
                }
                unsafe { __getSlotRing(self) }
            }
        }
        impl Context {
            /// @brief Getter method to the index set to the small primes.
            /// @return A `const` reference to the index set to the small primes.
            pub fn getSmallPrimes(&self) -> &IndexSet {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getSmallPrimes"]
                    fn __getSmallPrimes(_: &Context) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { &*__getSmallPrimes(self).cast() }
            }
        }
        impl Context {
            /// @brief Getter method to the index set to the ciphertext primes.
            /// @return A `const` reference to the index set to the ciphertext primes.
            pub fn getCtxtPrimes(&self) -> &IndexSet {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getCtxtPrimes"]
                    fn __getCtxtPrimes(_: &Context) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { &*__getCtxtPrimes(self).cast() }
            }
        }
        impl Context {
            /// @brief Getter method to the index set to the special primes.
            /// @return A `const` reference to the index set to the special primes.
            pub fn getSpecialPrimes(&self) -> &IndexSet {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getSpecialPrimes"]
                    fn __getSpecialPrimes(_: &Context) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { &*__getSpecialPrimes(self).cast() }
            }
        }
        impl<'a> Context {
            /// @brief Getter method to the digits.
            /// @return A `const` reference to a `std::vector` of index sets that
            /// represent the digits.
            pub fn getDigits(&'a self) -> &'a ::cxx::CxxVector<IndexSet> {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getDigits"]
                    fn __getDigits<'a>(_: &'a Context) -> &'a ::cxx::CxxVector<IndexSet>;
                }
                unsafe { __getDigits(self) }
            }
        }
        impl Context {
            /// @brief Getter method to get a single digit.
            /// @param i The `i` the digit.
            /// @return A `const` reference to an index set that representing the `i`th
            /// digit.
            pub fn getDigit(&self, i: c_long) -> &IndexSet {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getDigit"]
                    fn __getDigit(_: &Context, i: *mut c_long) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe {
                    let mut i = ::cxx::core::mem::MaybeUninit::new(i);
                    &*__getDigit(self, i.as_mut_ptr()).cast()
                }
            }
        }
        impl Context {
            /// @brief Getter method for a recryption data object.
            /// @return A `const` reference to the recryption data object.
            pub fn getRcData(&self) -> &ThinRecryptData {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getRcData"]
                    fn __getRcData(_: &Context) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { &*__getRcData(self).cast() }
            }
        }
        impl Context {
            /// @brief Return whether this is a CKKS context or not `Context`.
            /// @return A `bool`, `true` if the `Context` object uses CKKS scheme false
            /// otherwise.
            /// @note We assume `false` return to be BGV scheme.
            pub fn isCKKS(&self) -> bool {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$isCKKS"]
                    fn __isCKKS(_: &Context) -> bool;
                }
                unsafe { __isCKKS(self) }
            }
        }
        impl Context {
            /// @brief Getter method for the Hamming weight value.
            /// @return The Hamming weight value.
            pub fn getHwt(&self) -> c_long {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getHwt"]
                    fn __getHwt(_: &Context, __return: *mut c_long);
                }
                unsafe {
                    let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                    __getHwt(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        impl Context {
            /// @brief Getter method for the e parameter.
            /// @return The e parameter.
            pub fn getE(&self) -> c_long {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getE"]
                    fn __getE(_: &Context, __return: *mut c_long);
                }
                unsafe {
                    let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                    __getE(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        impl Context {
            /// @brief Getter method for the e prime parameter.
            /// @return The e prime parameter.
            pub fn getEPrime(&self) -> c_long {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getEPrime"]
                    fn __getEPrime(_: &Context, __return: *mut c_long);
                }
                unsafe {
                    let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                    __getEPrime(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        /// @brief Get the underlying `zMStar` object.
        /// @return A `zMStar` object.
        pub fn helib_Context_getZMStar_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
        ) -> &PAlgebra {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_getZMStar_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_getZMStar_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                ) -> *const ::cxx::core::ffi::c_void;
            }
            unsafe {
                &*__helib_Context_getZMStar_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                )
                .cast()
            }
        }
        impl Context {
            /// @brief Get the underlying `AlMod` object.
            /// @return A `AlMod` object.
            pub fn getAlMod(&self) -> &PAlgebraMod {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getAlMod"]
                    fn __getAlMod(_: &Context) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { &*__getAlMod(self).cast() }
            }
        }
        impl Context {
            /// @brief Getter method returning the default `view` object of the created
            /// `context`.
            /// @return A raw pointer/reference to the `EncryptedArray` object.
            pub fn getView(&self) -> &EncryptedArray {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getView"]
                    fn __getView(_: &Context) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { &*__getView(self).cast() }
            }
        }
        impl Context {
            /// @brief Getter method returning the default `EncryptedArray` object of the
            ///created `context`.
            /// @return A raw pointer/reference to the `EncryptedArray` object.
            /// @note It is foreseen that this method will be eventually deprecated in
            /// favour of the alternative `getView`.
            pub fn getEA(&self) -> &EncryptedArray {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getEA"]
                    fn __getEA(_: &Context) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { &*__getEA(self).cast() }
            }
        }
        impl Context {
            /// @brief Transfer ownership of the `EncryptedArray` object to the caller.
            ///
            /// This function returns a unique pointer to the `EncryptedArray`, which signifies
            /// the transfer of ownership. After calling this function, the Context object
            /// no longer has ownership of the `EncryptedArray`.
            ///
            /// @note Workaround for google/autocxx#799 & dtolnay/cxx#850.
            ///
            /// @return A unique pointer to the `EncryptedArray`.
            pub fn getHandle(
                self: ::cxx::core::pin::Pin<&mut Self>,
            ) -> ::cxx::UniquePtr<EncryptedArray> {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getHandle"]
                    fn __getHandle(
                        _: ::cxx::core::pin::Pin<&mut Context>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { ::cxx::UniquePtr::from_raw(__getHandle(self).cast()) }
            }
        }
        impl Context {
            /// @brief The `getHandle` method is the preferred way for an external
            /// language binding to interact with the `EncryptedArray` object of the
            /// created `context`.
            /// However, the `getUseCount` function can still be valuable for debugging
            /// or verification purposes.
            /// For instance, the `getUseCount` function is a direct way to confirm
            /// there are no unexpected references (or the lack thereof) to an object.
            /// @note This is mainly for debugging or verification and doesn't directly
            /// contribute to safe usage in external language bindings.
            /// @return Use count of the `std::shared_ptr` to the `EncryptedArray` object.
            pub fn getUseCount(&self) -> i64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getUseCount"]
                    fn __getUseCount(_: &Context) -> i64;
                }
                unsafe { __getUseCount(self) }
            }
        }
        impl Context {
            pub fn noiseBoundForUniform(&self, magBound: f64, degBound: c_long) -> f64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$noiseBoundForUniform"]
                    fn __noiseBoundForUniform(
                        _: &Context,
                        magBound: f64,
                        degBound: *mut c_long,
                    ) -> f64;
                }
                unsafe {
                    let mut degBound = ::cxx::core::mem::MaybeUninit::new(degBound);
                    __noiseBoundForUniform(self, magBound, degBound.as_mut_ptr())
                }
            }
        }
        pub unsafe fn noiseBoundForUniform1_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
            magBound: *mut xdouble,
            degBound: c_long,
            placement_return_type: *mut xdouble,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$noiseBoundForUniform1_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __noiseBoundForUniform1_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    magBound: *mut ::cxx::core::ffi::c_void,
                    degBound: *mut c_long,
                    placement_return_type: *mut ::cxx::core::ffi::c_void,
                );
            }
            let mut degBound = ::cxx::core::mem::MaybeUninit::new(degBound);
            __noiseBoundForUniform1_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                magBound.cast(),
                degBound.as_mut_ptr(),
                placement_return_type.cast(),
            )
        }
        impl Context {
            pub fn noiseBoundForMod(&self, modulus: c_long, degBound: c_long) -> f64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$noiseBoundForMod"]
                    fn __noiseBoundForMod(
                        _: &Context,
                        modulus: *mut c_long,
                        degBound: *mut c_long,
                    ) -> f64;
                }
                unsafe {
                    let mut modulus = ::cxx::core::mem::MaybeUninit::new(modulus);
                    let mut degBound = ::cxx::core::mem::MaybeUninit::new(degBound);
                    __noiseBoundForMod(self, modulus.as_mut_ptr(), degBound.as_mut_ptr())
                }
            }
        }
        impl Context {
            pub fn noiseBoundForGaussian(&self, sigma: f64, degBound: c_long) -> f64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$noiseBoundForGaussian"]
                    fn __noiseBoundForGaussian(
                        _: &Context,
                        sigma: f64,
                        degBound: *mut c_long,
                    ) -> f64;
                }
                unsafe {
                    let mut degBound = ::cxx::core::mem::MaybeUninit::new(degBound);
                    __noiseBoundForGaussian(self, sigma, degBound.as_mut_ptr())
                }
            }
        }
        pub unsafe fn noiseBoundForGaussian1_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
            sigma: *mut xdouble,
            degBound: c_long,
            placement_return_type: *mut xdouble,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$noiseBoundForGaussian1_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __noiseBoundForGaussian1_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    sigma: *mut ::cxx::core::ffi::c_void,
                    degBound: *mut c_long,
                    placement_return_type: *mut ::cxx::core::ffi::c_void,
                );
            }
            let mut degBound = ::cxx::core::mem::MaybeUninit::new(degBound);
            __noiseBoundForGaussian1_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                sigma.cast(),
                degBound.as_mut_ptr(),
                placement_return_type.cast(),
            )
        }
        impl Context {
            pub fn noiseBoundForSmall(&self, prob: f64, degBound: c_long) -> f64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$noiseBoundForSmall"]
                    fn __noiseBoundForSmall(_: &Context, prob: f64, degBound: *mut c_long) -> f64;
                }
                unsafe {
                    let mut degBound = ::cxx::core::mem::MaybeUninit::new(degBound);
                    __noiseBoundForSmall(self, prob, degBound.as_mut_ptr())
                }
            }
        }
        impl Context {
            pub fn noiseBoundForHWt(&self, hwt: c_long, degBound: c_long) -> f64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$noiseBoundForHWt"]
                    fn __noiseBoundForHWt(
                        _: &Context,
                        hwt: *mut c_long,
                        degBound: *mut c_long,
                    ) -> f64;
                }
                unsafe {
                    let mut hwt = ::cxx::core::mem::MaybeUninit::new(hwt);
                    let mut degBound = ::cxx::core::mem::MaybeUninit::new(degBound);
                    __noiseBoundForHWt(self, hwt.as_mut_ptr(), degBound.as_mut_ptr())
                }
            }
        }
        impl Context {
            /// @brief Calculate the standard deviation for recryption.
            /// @return The standard deviation for recryption.
            pub fn stdDevForRecryption(&self) -> f64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$stdDevForRecryption"]
                    fn __stdDevForRecryption(_: &Context) -> f64;
                }
                unsafe { __stdDevForRecryption(self) }
            }
        }
        impl Context {
            /// @brief Calculate the bound for recryption.
            /// @return The bound for recryption.
            pub fn boundForRecryption(&self) -> f64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$boundForRecryption"]
                    fn __boundForRecryption(_: &Context) -> f64;
                }
                unsafe { __boundForRecryption(self) }
            }
        }
        impl Context {
            /// @brief Get the helper table to map required modulo-sizes to primeSets.
            /// @return The table as `ModuliSizes` type.
            pub fn getModSizeTable(&self) -> &ModuliSizes {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$getModSizeTable"]
                    fn __getModSizeTable(_: &Context) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe { &*__getModSizeTable(self).cast() }
            }
        }
        impl Context {
            /// @brief Set the helper table to map required modulo-sizes to primeSets.
            pub fn setModSizeTable(self: ::cxx::core::pin::Pin<&mut Self>) {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$setModSizeTable"]
                    fn __setModSizeTable(_: ::cxx::core::pin::Pin<&mut Context>);
                }
                unsafe { __setModSizeTable(self) }
            }
        }
        impl Context {
            /// @brief Initialises the recryption data.
            /// @param mvec A `std::vector` of unique prime factors of `m`.
            /// @param build_cache Flag for building a cache for improved efficiency.
            /// Default is false.
            /// @param alsoThick Flag for initialising additional information needed for
            /// thick bootstrapping. Default is true.
            pub fn enableBootStrapping(
                self: ::cxx::core::pin::Pin<&mut Self>,
                mvec: &NTL_Vec_long_AutocxxConcrete,
                build_cache: bool,
                alsoThick: bool,
            ) {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$enableBootStrapping"]
                    fn __enableBootStrapping(
                        _: ::cxx::core::pin::Pin<&mut Context>,
                        mvec: &NTL_Vec_long_AutocxxConcrete,
                        build_cache: bool,
                        alsoThick: bool,
                    );
                }
                unsafe { __enableBootStrapping(self, mvec, build_cache, alsoThick) }
            }
        }
        impl Context {
            /// @brief Check if a `Context` is bootstrappable.
            /// @return `true` if recryption data is found, `false` otherwise.
            pub fn isBootstrappable(&self) -> bool {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$isBootstrappable"]
                    fn __isBootstrappable(_: &Context) -> bool;
                }
                unsafe { __isBootstrappable(self) }
            }
        }
        /// @brief Getter method that returns the handles of both the `ctxtPrimes` and
        /// `specialPrimes` associated with this `Context`.
        /// @return `IndexSet` of the handles to the `ctxtPrimes` and `specialPrimes`.
        pub unsafe fn fullPrimes_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
            placement_return_type: *mut IndexSet,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$fullPrimes_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __fullPrimes_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    placement_return_type: *mut ::cxx::core::ffi::c_void,
                );
            }
            __fullPrimes_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                placement_return_type.cast(),
            )
        }
        /// @brief Getter method that returns the handles of all primes associated with
        /// this `Context`.
        /// @return `IndexSet` of handles to the `ctxtPrimes`, `specialPrimes` and
        /// `smallPrimes`.
        pub unsafe fn allPrimes_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
            placement_return_type: *mut IndexSet,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$allPrimes_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __allPrimes_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    placement_return_type: *mut ::cxx::core::ffi::c_void,
                );
            }
            __allPrimes_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                placement_return_type.cast(),
            )
        }
        /// @brief Getter method that returns the first `nprimes` `ctxtPrimes`
        /// associated with this `Context`.
        /// @param nprimes The number of desired `ctxtPrimes`.
        /// @return `IndexSet` of handles to the first `nprimes` `ctxtPrimes`.
        pub unsafe fn getCtxtPrimes1_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
            nprimes: c_long,
            placement_return_type: *mut IndexSet,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$getCtxtPrimes1_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __getCtxtPrimes1_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    nprimes: *mut c_long,
                    placement_return_type: *mut ::cxx::core::ffi::c_void,
                );
            }
            let mut nprimes = ::cxx::core::mem::MaybeUninit::new(nprimes);
            __getCtxtPrimes1_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                nprimes.as_mut_ptr(),
                placement_return_type.cast(),
            )
        }
        impl Context {
            pub fn BPL(&self) -> c_long {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$BPL"]
                    fn __BPL(_: &Context, __return: *mut c_long);
                }
                unsafe {
                    let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                    __BPL(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        impl Context {
            /// @brief Getter method for the small prime of the modulus chain at index
            /// `i` as a `long`.
            /// @param i Index of the desired small prime.
            /// @return The small prime of the modulus chain at index `i`.
            pub fn ithPrime(&self, i: c_ulong) -> c_long {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$ithPrime"]
                    fn __ithPrime(_: &Context, i: *mut c_ulong, __return: *mut c_long);
                }
                unsafe {
                    let mut i = ::cxx::core::mem::MaybeUninit::new(i);
                    let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                    __ithPrime(self, i.as_mut_ptr(), __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        impl Context {
            /// @brief Getter method for the small prime of the modulus chain at index
            /// `i` as a `Cmodulus`.
            /// @param i Index of the desired small prime.
            /// @return Reference to the small prime modulus at index `i`.
            pub fn ithModulus(&self, i: c_ulong) -> &Cmodulus {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$ithModulus"]
                    fn __ithModulus(
                        _: &Context,
                        i: *mut c_ulong,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                unsafe {
                    let mut i = ::cxx::core::mem::MaybeUninit::new(i);
                    &*__ithModulus(self, i.as_mut_ptr()).cast()
                }
            }
        }
        impl Context {
            /// @brief Return the total number of small primes in the modulus chain.
            /// @return The total number of small primes in the modulus chain.
            pub fn numPrimes(&self) -> c_long {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$numPrimes"]
                    fn __numPrimes(_: &Context, __return: *mut c_long);
                }
                unsafe {
                    let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                    __numPrimes(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        impl Context {
            /// @brief Check if a number is divisible by any of the primes in the modulus
            /// chain.
            /// @param num The number to check.
            /// @return `true` if the modulus chain contains at least one divisor of
            /// `num`, false otherwise.
            pub fn isZeroDivisor(&self, num: &ZZ) -> bool {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$isZeroDivisor"]
                    fn __isZeroDivisor(_: &Context, num: *const ::cxx::core::ffi::c_void) -> bool;
                }
                unsafe {
                    __isZeroDivisor(self, num as *const ZZ as *const ::cxx::core::ffi::c_void)
                }
            }
        }
        impl Context {
            /// @brief Check if value is already contained within the modulus chain.
            /// @param p The number to check.
            /// @return `true` if `p` is already contained within the modulus chain,
            /// `false` otherwise.
            pub fn inChain(&self, p: c_long) -> bool {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$inChain"]
                    fn __inChain(_: &Context, p: *mut c_long) -> bool;
                }
                unsafe {
                    let mut p = ::cxx::core::mem::MaybeUninit::new(p);
                    __inChain(self, p.as_mut_ptr())
                }
            }
        }
        impl Context {
            /// @brief Calculate the product of all primes in the given set.
            /// @param p The product of the input primes.
            /// @param s The set of input primes to the product.
            pub fn productOfPrimes(&self, p: ::cxx::core::pin::Pin<&mut ZZ>, s: &IndexSet) {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$productOfPrimes"]
                    fn __productOfPrimes(
                        _: &Context,
                        p: *mut ::cxx::core::ffi::c_void,
                        s: *const ::cxx::core::ffi::c_void,
                    );
                }
                unsafe {
                    __productOfPrimes(
                        self,
                        ::cxx::core::pin::Pin::into_inner_unchecked(p) as *mut ZZ
                            as *mut ::cxx::core::ffi::c_void,
                        s as *const IndexSet as *const ::cxx::core::ffi::c_void,
                    )
                }
            }
        }
        pub unsafe fn productOfPrimes1_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
            s: &IndexSet,
            placement_return_type: *mut ZZ,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$productOfPrimes1_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __productOfPrimes1_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    s: *const ::cxx::core::ffi::c_void,
                    placement_return_type: *mut ::cxx::core::ffi::c_void,
                );
            }
            __productOfPrimes1_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                s as *const IndexSet as *const ::cxx::core::ffi::c_void,
                placement_return_type.cast(),
            )
        }
        impl Context {
            /// @brief Calculate the natural logarithm of the `i`th prime of the modulus
            /// chain.
            /// @param i Index of the desired prime.
            /// @return The natural logarithm of the `i`th prime of the modulus chain.
            pub fn logOfPrime(&self, i: c_ulong) -> f64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$logOfPrime"]
                    fn __logOfPrime(_: &Context, i: *mut c_ulong) -> f64;
                }
                unsafe {
                    let mut i = ::cxx::core::mem::MaybeUninit::new(i);
                    __logOfPrime(self, i.as_mut_ptr())
                }
            }
        }
        impl Context {
            /// @brief Calculate the natural logarithm of `productOfPrimes(s)` for a given
            /// set of primes `s`.
            /// @param s The set of input primes.
            /// @return The natural logarithm of the product of the input primes.
            pub fn logOfProduct(&self, s: &IndexSet) -> f64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$logOfProduct"]
                    fn __logOfProduct(_: &Context, s: *const ::cxx::core::ffi::c_void) -> f64;
                }
                unsafe {
                    __logOfProduct(
                        self,
                        s as *const IndexSet as *const ::cxx::core::ffi::c_void,
                    )
                }
            }
        }
        impl Context {
            /// @brief Calculate the size of the ciphertext modulus `Q` in bits.
            /// @return The bit size of the ciphertext modulus `Q = ctxtPrimes |
            /// specialPrimes`.
            pub fn bitSizeOfQ(&self) -> c_long {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$bitSizeOfQ"]
                    fn __bitSizeOfQ(_: &Context, __return: *mut c_long);
                }
                unsafe {
                    let mut __return = ::cxx::core::mem::MaybeUninit::<c_long>::uninit();
                    __bitSizeOfQ(self, __return.as_mut_ptr());
                    __return.assume_init()
                }
            }
        }
        impl Context {
            /// @brief An estimate for the security-level. This has a lower bound of 0.
            /// @param hwt The Hamming weight of the secret key.
            ///
            /// @note This function uses experimental affine approximations to the
            /// lwe-estimator from
            /// https://bitbucket.org/malb/lwe-estimator/raw/HEAD/estimator.py, from
            /// Aug-2020 (see script in misc/estimator/lwe-estimator.sage).
            ///
            /// Let s=3.2 if m is a power of two, or s=3.2*sqrt(m) otherwise. For the
            /// estimator we use alpha=s/q (so log2AlphaInv = log_2(q/s)), and n=phi(m).
            pub fn securityLevel(&self) -> f64 {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$securityLevel"]
                    fn __securityLevel(_: &Context) -> f64;
                }
                unsafe { __securityLevel(self) }
            }
        }
        /// @brief Print out algebra and other important info
        /// @param out Output `std::ostream`.
        pub fn helib_Context_printout_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
            out: ::cxx::core::pin::Pin<&mut std_basic_ostream_char_AutocxxConcrete>,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_printout_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_printout_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    out: ::cxx::core::pin::Pin<&mut std_basic_ostream_char_AutocxxConcrete>,
                );
            }
            unsafe {
                __helib_Context_printout_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                    out,
                )
            }
        }
        /// @brief Write out the `Context` object in binary format.
        /// @param str Output `std::ostream`.
        pub fn helib_Context_writeTo_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
            str_: ::cxx::core::pin::Pin<&mut std_basic_ostream_char_AutocxxConcrete>,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_writeTo_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_writeTo_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    str_: ::cxx::core::pin::Pin<&mut std_basic_ostream_char_AutocxxConcrete>,
                );
            }
            unsafe {
                __helib_Context_writeTo_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                    str_,
                )
            }
        }
        /// @brief Read from the stream the serialized `Context` object in binary
        /// format.
        /// @param str Input `std::istream`.
        /// @return The deserialized `Context` object.
        pub unsafe fn helib_Context_readFrom_autocxx_wrapper_0xe97f7296bea4f464(
            str_: ::cxx::core::pin::Pin<&mut std_basic_istream_char_AutocxxConcrete>,
            placement_return_type: *mut Context,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_readFrom_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_readFrom_autocxx_wrapper_0xe97f7296bea4f464(
                    str_: ::cxx::core::pin::Pin<&mut std_basic_istream_char_AutocxxConcrete>,
                    placement_return_type: *mut ::cxx::core::ffi::c_void,
                );
            }
            __helib_Context_readFrom_autocxx_wrapper_0xe97f7296bea4f464(
                str_,
                placement_return_type.cast(),
            )
        }
        /// @brief Read from the stream the serialized `Context` object in binary
        /// format.
        /// @param str Input `std::istream`.
        /// @return Raw pointer to the deserialized `Context` object.
        pub fn readPtrFrom_autocxx_wrapper_0xe97f7296bea4f464(
            str_: ::cxx::core::pin::Pin<&mut std_basic_istream_char_AutocxxConcrete>,
        ) -> *mut Context {
            extern "C" {
                #[link_name = "cxxbridge1$readPtrFrom_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __readPtrFrom_autocxx_wrapper_0xe97f7296bea4f464(
                    str_: ::cxx::core::pin::Pin<&mut std_basic_istream_char_AutocxxConcrete>,
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            unsafe { __readPtrFrom_autocxx_wrapper_0xe97f7296bea4f464(str_).cast() }
        }
        /// @brief Write out the `Context` object to the output stream using JSON
        /// format.
        /// @param str Output `std::ostream`.
        pub fn helib_Context_writeToJSON_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
            str_: ::cxx::core::pin::Pin<&mut std_basic_ostream_char_AutocxxConcrete>,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_writeToJSON_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_writeToJSON_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    str_: ::cxx::core::pin::Pin<&mut std_basic_ostream_char_AutocxxConcrete>,
                );
            }
            unsafe {
                __helib_Context_writeToJSON_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                    str_,
                )
            }
        }
        /// @brief Write out the `Context` object to a `JsonWrapper`.
        /// @return The `JsonWrapper`.
        pub unsafe fn helib_Context_writeToJSON1_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: &Context,
            placement_return_type: *mut JsonWrapper,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_writeToJSON1_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_writeToJSON1_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                    placement_return_type: *mut ::cxx::core::ffi::c_void,
                );
            }
            __helib_Context_writeToJSON1_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this as *const Context as *const ::cxx::core::ffi::c_void,
                placement_return_type.cast(),
            )
        }
        /// @brief Read from the stream the serialized `Context` object using JSON
        /// format.
        /// @param str Input `std::istream`.
        /// @return The deserialized `Context` object.
        pub unsafe fn helib_Context_readFromJSON_autocxx_wrapper_0xe97f7296bea4f464(
            str_: ::cxx::core::pin::Pin<&mut std_basic_istream_char_AutocxxConcrete>,
            placement_return_type: *mut Context,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_readFromJSON_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_readFromJSON_autocxx_wrapper_0xe97f7296bea4f464(
                    str_: ::cxx::core::pin::Pin<&mut std_basic_istream_char_AutocxxConcrete>,
                    placement_return_type: *mut ::cxx::core::ffi::c_void,
                );
            }
            __helib_Context_readFromJSON_autocxx_wrapper_0xe97f7296bea4f464(
                str_,
                placement_return_type.cast(),
            )
        }
        /// @brief Read from the `JsonWrapper` the serialized `Context` object.
        /// @param j The `JsonWrapper` containing the serialized `Context` object.
        /// @return The deserialized `Context` object.
        pub unsafe fn helib_Context_readFromJSON1_autocxx_wrapper_0xe97f7296bea4f464(
            j: &JsonWrapper,
            placement_return_type: *mut Context,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Context_readFromJSON1_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Context_readFromJSON1_autocxx_wrapper_0xe97f7296bea4f464(
                    j: *const ::cxx::core::ffi::c_void,
                    placement_return_type: *mut ::cxx::core::ffi::c_void,
                );
            }
            __helib_Context_readFromJSON1_autocxx_wrapper_0xe97f7296bea4f464(
                j as *const JsonWrapper as *const ::cxx::core::ffi::c_void,
                placement_return_type.cast(),
            )
        }
        /// @brief Read from the `JsonWrapper` the serialized `Context` object.
        /// @param j The `JsonWrapper` containing the serialized `Context` object.
        /// @return Raw pointer to the deserialized `Context` object.
        pub fn readPtrFromJSON_autocxx_wrapper_0xe97f7296bea4f464(
            str_: ::cxx::core::pin::Pin<&mut std_basic_istream_char_AutocxxConcrete>,
        ) -> *mut Context {
            extern "C" {
                #[link_name = "cxxbridge1$readPtrFromJSON_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __readPtrFromJSON_autocxx_wrapper_0xe97f7296bea4f464(
                    str_: ::cxx::core::pin::Pin<&mut std_basic_istream_char_AutocxxConcrete>,
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            unsafe { __readPtrFromJSON_autocxx_wrapper_0xe97f7296bea4f464(str_).cast() }
        }
        impl Context {
            pub fn clearModChain(self: ::cxx::core::pin::Pin<&mut Self>) {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$clearModChain"]
                    fn __clearModChain(_: ::cxx::core::pin::Pin<&mut Context>);
                }
                unsafe { __clearModChain(self) }
            }
        }
        impl Context {
            /// @brief Build the modulus chain for given `Context` object.
            /// @param nBits Total number of bits required for the modulus chain.
            /// @param nDgts Number of digits/columns in the key-switching matrix. Default
            /// is 3.
            /// @param willBeBoostrappable Flag for initializing bootstrapping data.
            ///Default is `false`.
            /// @param skHwt The Hamming weight of the secret key. Default is 0.
            /// @param resolution The bit size of resolution of the modulus chain. Default
            /// is 3.
            /// @param bitsInSpecialPrimes The bit size of the special primes in the
            ///modulus chain. Default is 0.
            pub fn buildModChain(
                self: ::cxx::core::pin::Pin<&mut Self>,
                nBits: c_long,
                nDgts: c_long,
                willBeBootstrappable: bool,
                skHwt: c_long,
                resolution: c_long,
                bitsInSpecialPrimes: c_long,
            ) {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$buildModChain"]
                    fn __buildModChain(
                        _: ::cxx::core::pin::Pin<&mut Context>,
                        nBits: *mut c_long,
                        nDgts: *mut c_long,
                        willBeBootstrappable: bool,
                        skHwt: *mut c_long,
                        resolution: *mut c_long,
                        bitsInSpecialPrimes: *mut c_long,
                    );
                }
                unsafe {
                    let mut nBits = ::cxx::core::mem::MaybeUninit::new(nBits);
                    let mut nDgts = ::cxx::core::mem::MaybeUninit::new(nDgts);
                    let mut skHwt = ::cxx::core::mem::MaybeUninit::new(skHwt);
                    let mut resolution = ::cxx::core::mem::MaybeUninit::new(resolution);
                    let mut bitsInSpecialPrimes =
                        ::cxx::core::mem::MaybeUninit::new(bitsInSpecialPrimes);
                    __buildModChain(
                        self,
                        nBits.as_mut_ptr(),
                        nDgts.as_mut_ptr(),
                        willBeBootstrappable,
                        skHwt.as_mut_ptr(),
                        resolution.as_mut_ptr(),
                        bitsInSpecialPrimes.as_mut_ptr(),
                    )
                }
            }
        }
        impl Context {
            pub fn endBuildModChain(self: ::cxx::core::pin::Pin<&mut Self>) {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$Context$endBuildModChain"]
                    fn __endBuildModChain(_: ::cxx::core::pin::Pin<&mut Context>);
                }
                unsafe { __endBuildModChain(self) }
            }
        }
        /// @brief Default destructor.
        pub unsafe fn Context_destructor_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: *mut Context,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$Context_destructor_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __Context_destructor_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                );
            }
            __Context_destructor_autocxx_wrapper_0xe97f7296bea4f464(autocxx_gen_this.cast())
        }
        pub type xdouble = super::bindgen::root::NTL::xdouble;
        /// @class PowerfulDCRT
        /// @brief Conversion between powerful representation, DoubleCRT, and ZZX
        pub type PowerfulDCRT = super::bindgen::root::helib::PowerfulDCRT;
        /// @struct PolyModRing
        /// @brief Lightweight type for describing the structure of a single slot of the
        /// plaintext space.
        ///
        /// A single slot of the plaintext space is isomorphic to
        /// \f$\mathbb{Z}[X]/(G(x),p^r)\f$ for some irreducible factor G of
        /// \f$\Phi_m(X)\f$, so the main useful members of this `struct` are `p`, `r`,
        /// `G`, and `p2r`.
        ///
        /// The fields of this `struct` are all `const`, so they should be determined
        /// at the time of construction.
        ///
        /// @note This `struct` aggregates this often-useful information into a single
        /// placeholder for convenience.
        pub type PolyModRing = super::bindgen::root::helib::PolyModRing;
        ///! @brief A dynamic set of non-negative integers.
        ///!
        ///! You can iterate through a set as follows:
        ///! \code
        ///!    for (long i = s.first(); i <= s.last(); i = s.next(i)) ...
        ///!    for (long i = s.last(); i >= s.first(); i = s.prev(i)) ...
        ///! \endcode
        pub type IndexSet = super::bindgen::root::helib::IndexSet;
        ///! @class ThinRecryptData
        ///! @brief Same as above, but for "thin" bootstrapping, where the slots
        ///! are assumed to contain constants
        pub type ThinRecryptData = super::bindgen::root::helib::ThinRecryptData;
        /// @class PAlgebra
        /// @brief The structure of (Z/mZ)* /(p)
        ///
        /// A PAlgebra object is determined by an integer m and a prime p, where p does
        /// not divide m. It holds information describing the structure of (Z/mZ)^*,
        /// which is isomorphic to the Galois group over A = Z[X]/Phi_m(X)).
        ///
        /// We represent (Z/mZ)^* as (Z/mZ)^* = (p) x (g1,g2,...) x (h1,h2,...)
        /// where the group generated by g1,g2,... consists of the elements that
        /// have the same order in (Z/mZ)^* as in (Z/mZ)^* /(p,g_1,...,g_{i-1}), and
        /// h1,h2,... generate the remaining quotient group (Z/mZ)^* /(p,g1,g2,...).
        ///
        /// We let T subset (Z/mZ)^* be a set of representatives for the quotient
        /// group (Z/mZ)^* /(p), defined as T={ prod_i gi^{ei} * prod_j hj^{ej} }
        /// where the ei's range over 0,1,...,ord(gi)-1 and the ej's range over
        /// 0,1,...ord(hj)-1 (these last orders are in (Z/mZ)^* /(p,g1,g2,...)).
        ///
        /// Phi_m(X) is factored as Phi_m(X)= prod_{t in T} F_t(X) mod p,
        /// where the F_t's are irreducible modulo p. An arbitrary factor
        /// is chosen as F_1, then for each t in T we associate with the index t the
        /// factor F_t(X) = GCD(F_1(X^t), Phi_m(X)).
        ///
        /// Note that fixing a representation of the field R=(Z/pZ)[X]/F_1(X)
        /// and letting z be a root of F_1 in R (which
        /// is a primitive m-th root of unity in R), we get that F_t is the minimal
        /// polynomial of z^{1/t}.
        pub type PAlgebra = super::bindgen::root::helib::PAlgebra;
        pub type PAlgebraMod = super::bindgen::root::helib::PAlgebraMod;
        ///! @class EncryptedArray
        ///! @brief A simple wrapper for a smart pointer to an EncryptedArrayBase.
        ///! This is the interface that higher-level code should use
        pub type EncryptedArray = super::bindgen::root::helib::EncryptedArray;
        ///! A helper class to map required modulo-sizes to primeSets
        pub type ModuliSizes = super::bindgen::root::helib::ModuliSizes;
        #[repr(C)]
        pub struct NTL_Vec_long_AutocxxConcrete {
            _private: ::cxx::private::Opaque,
        }
        unsafe impl ::cxx::ExternType for NTL_Vec_long_AutocxxConcrete {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::N,
                ::cxx::T,
                ::cxx::L,
                ::cxx::__,
                ::cxx::V,
                ::cxx::e,
                ::cxx::c,
                ::cxx::__,
                ::cxx::l,
                ::cxx::o,
                ::cxx::n,
                ::cxx::g,
                ::cxx::__,
                ::cxx::A,
                ::cxx::u,
                ::cxx::t,
                ::cxx::o,
                ::cxx::c,
                ::cxx::x,
                ::cxx::x,
                ::cxx::C,
                ::cxx::o,
                ::cxx::n,
                ::cxx::c,
                ::cxx::r,
                ::cxx::e,
                ::cxx::t,
                ::cxx::e,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        /// @class Cmodulus
        /// @brief Provides FFT and iFFT routines modulo a single-precision prime
        ///
        /// On initialization, it initializes NTL's zz_pContext for this q
        /// and computes a 2m-th root of unity r mod q and also r^{-1} mod q.
        /// Thereafter this class provides FFT and iFFT routines that converts between
        /// time & frequency domains. Some tables are computed the first time that
        /// each directions is called, which are then used in subsequent computations.
        ///
        /// The "time domain" polynomials are represented as ZZX, which are reduced
        /// modulo Phi_m(X). The "frequency domain" are just vectors of integers
        /// (vec_long), that store only the evaluation in primitive m-th
        /// roots of unity.
        pub type Cmodulus = super::bindgen::root::helib::Cmodulus;
        pub type ZZ = super::bindgen::root::NTL::ZZ;
        #[repr(C)]
        pub struct std_basic_ostream_char_AutocxxConcrete {
            _private: ::cxx::private::Opaque,
        }
        unsafe impl ::cxx::ExternType for std_basic_ostream_char_AutocxxConcrete {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::s,
                ::cxx::t,
                ::cxx::d,
                ::cxx::__,
                ::cxx::b,
                ::cxx::a,
                ::cxx::s,
                ::cxx::i,
                ::cxx::c,
                ::cxx::__,
                ::cxx::o,
                ::cxx::s,
                ::cxx::t,
                ::cxx::r,
                ::cxx::e,
                ::cxx::a,
                ::cxx::m,
                ::cxx::__,
                ::cxx::c,
                ::cxx::h,
                ::cxx::a,
                ::cxx::r,
                ::cxx::__,
                ::cxx::A,
                ::cxx::u,
                ::cxx::t,
                ::cxx::o,
                ::cxx::c,
                ::cxx::x,
                ::cxx::x,
                ::cxx::C,
                ::cxx::o,
                ::cxx::n,
                ::cxx::c,
                ::cxx::r,
                ::cxx::e,
                ::cxx::t,
                ::cxx::e,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        #[repr(C)]
        pub struct std_basic_istream_char_AutocxxConcrete {
            _private: ::cxx::private::Opaque,
        }
        unsafe impl ::cxx::ExternType for std_basic_istream_char_AutocxxConcrete {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::s,
                ::cxx::t,
                ::cxx::d,
                ::cxx::__,
                ::cxx::b,
                ::cxx::a,
                ::cxx::s,
                ::cxx::i,
                ::cxx::c,
                ::cxx::__,
                ::cxx::i,
                ::cxx::s,
                ::cxx::t,
                ::cxx::r,
                ::cxx::e,
                ::cxx::a,
                ::cxx::m,
                ::cxx::__,
                ::cxx::c,
                ::cxx::h,
                ::cxx::a,
                ::cxx::r,
                ::cxx::__,
                ::cxx::A,
                ::cxx::u,
                ::cxx::t,
                ::cxx::o,
                ::cxx::c,
                ::cxx::x,
                ::cxx::x,
                ::cxx::C,
                ::cxx::o,
                ::cxx::n,
                ::cxx::c,
                ::cxx::r,
                ::cxx::e,
                ::cxx::t,
                ::cxx::e,
            );
            type Kind = ::cxx::kind::Opaque;
        }
        pub type JsonWrapper = super::bindgen::root::helib::JsonWrapper;
        pub unsafe fn xdouble_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut xdouble {
            extern "C" {
                #[link_name = "cxxbridge1$xdouble_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __xdouble_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __xdouble_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn xdouble_free_autocxx_wrapper_0xe97f7296bea4f464(arg0: *mut xdouble) {
            extern "C" {
                #[link_name = "cxxbridge1$xdouble_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __xdouble_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __xdouble_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        ///Synthesized copy constructor.
        pub unsafe fn NTL_xdouble_new_synthetic_const_copy_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: *mut xdouble,
            other: &xdouble,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$NTL_xdouble_new_synthetic_const_copy_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __NTL_xdouble_new_synthetic_const_copy_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *const ::cxx::core::ffi::c_void,
                );
            }
            __NTL_xdouble_new_synthetic_const_copy_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464 (autocxx_gen_this . cast () , other as * const xdouble as * const :: cxx :: core :: ffi :: c_void)
        }
        pub unsafe fn PowerfulDCRT_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut PowerfulDCRT {
            extern "C" {
                #[link_name = "cxxbridge1$PowerfulDCRT_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __PowerfulDCRT_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __PowerfulDCRT_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn PowerfulDCRT_free_autocxx_wrapper_0xe97f7296bea4f464(
            arg0: *mut PowerfulDCRT,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$PowerfulDCRT_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __PowerfulDCRT_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __PowerfulDCRT_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        pub unsafe fn PolyModRing_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut PolyModRing {
            extern "C" {
                #[link_name = "cxxbridge1$PolyModRing_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __PolyModRing_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __PolyModRing_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn PolyModRing_free_autocxx_wrapper_0xe97f7296bea4f464(arg0: *mut PolyModRing) {
            extern "C" {
                #[link_name = "cxxbridge1$PolyModRing_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __PolyModRing_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __PolyModRing_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        /// @brief Copy constructor.
        pub unsafe fn helib_PolyModRing_new1_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: *mut PolyModRing,
            other: &PolyModRing,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_PolyModRing_new1_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_PolyModRing_new1_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *const ::cxx::core::ffi::c_void,
                );
            }
            __helib_PolyModRing_new1_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this.cast(),
                other as *const PolyModRing as *const ::cxx::core::ffi::c_void,
            )
        }
        /// @brief Move constructor.
        pub unsafe fn helib_PolyModRing_new2_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: *mut PolyModRing,
            other: *mut PolyModRing,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_PolyModRing_new2_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_PolyModRing_new2_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *mut ::cxx::core::ffi::c_void,
                );
            }
            __helib_PolyModRing_new2_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this.cast(),
                other.cast(),
            )
        }
        pub unsafe fn IndexSet_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut IndexSet {
            extern "C" {
                #[link_name = "cxxbridge1$IndexSet_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __IndexSet_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __IndexSet_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn IndexSet_free_autocxx_wrapper_0xe97f7296bea4f464(arg0: *mut IndexSet) {
            extern "C" {
                #[link_name = "cxxbridge1$IndexSet_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __IndexSet_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __IndexSet_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        ///Synthesized move constructor.
        pub unsafe fn helib_IndexSet_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: *mut IndexSet,
            other: *mut IndexSet,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_IndexSet_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_IndexSet_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *mut ::cxx::core::ffi::c_void,
                );
            }
            __helib_IndexSet_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464 (autocxx_gen_this . cast () , other . cast ())
        }
        ///Synthesized copy constructor.
        pub unsafe fn helib_IndexSet_new_synthetic_const_copy_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: *mut IndexSet,
            other: &IndexSet,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_IndexSet_new_synthetic_const_copy_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_IndexSet_new_synthetic_const_copy_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *const ::cxx::core::ffi::c_void,
                );
            }
            __helib_IndexSet_new_synthetic_const_copy_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464 (autocxx_gen_this . cast () , other as * const IndexSet as * const :: cxx :: core :: ffi :: c_void)
        }
        pub unsafe fn ThinRecryptData_alloc_autocxx_wrapper_0xe97f7296bea4f464(
        ) -> *mut ThinRecryptData {
            extern "C" {
                #[link_name = "cxxbridge1$ThinRecryptData_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __ThinRecryptData_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __ThinRecryptData_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn ThinRecryptData_free_autocxx_wrapper_0xe97f7296bea4f464(
            arg0: *mut ThinRecryptData,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$ThinRecryptData_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __ThinRecryptData_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __ThinRecryptData_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        pub unsafe fn PAlgebra_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut PAlgebra {
            extern "C" {
                #[link_name = "cxxbridge1$PAlgebra_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __PAlgebra_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __PAlgebra_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn PAlgebra_free_autocxx_wrapper_0xe97f7296bea4f464(arg0: *mut PAlgebra) {
            extern "C" {
                #[link_name = "cxxbridge1$PAlgebra_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __PAlgebra_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __PAlgebra_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        pub unsafe fn PAlgebraMod_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut PAlgebraMod {
            extern "C" {
                #[link_name = "cxxbridge1$PAlgebraMod_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __PAlgebraMod_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __PAlgebraMod_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn PAlgebraMod_free_autocxx_wrapper_0xe97f7296bea4f464(arg0: *mut PAlgebraMod) {
            extern "C" {
                #[link_name = "cxxbridge1$PAlgebraMod_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __PAlgebraMod_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __PAlgebraMod_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        pub unsafe fn EncryptedArray_alloc_autocxx_wrapper_0xe97f7296bea4f464(
        ) -> *mut EncryptedArray {
            extern "C" {
                #[link_name = "cxxbridge1$EncryptedArray_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __EncryptedArray_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __EncryptedArray_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn EncryptedArray_free_autocxx_wrapper_0xe97f7296bea4f464(
            arg0: *mut EncryptedArray,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$EncryptedArray_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __EncryptedArray_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __EncryptedArray_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        pub unsafe fn ModuliSizes_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut ModuliSizes {
            extern "C" {
                #[link_name = "cxxbridge1$ModuliSizes_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __ModuliSizes_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __ModuliSizes_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn ModuliSizes_free_autocxx_wrapper_0xe97f7296bea4f464(arg0: *mut ModuliSizes) {
            extern "C" {
                #[link_name = "cxxbridge1$ModuliSizes_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __ModuliSizes_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __ModuliSizes_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        ///Synthesized move constructor.
        pub unsafe fn helib_ModuliSizes_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: *mut ModuliSizes,
            other: *mut ModuliSizes,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_ModuliSizes_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_ModuliSizes_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *mut ::cxx::core::ffi::c_void,
                );
            }
            __helib_ModuliSizes_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464 (autocxx_gen_this . cast () , other . cast ())
        }
        pub unsafe fn Cmodulus_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut Cmodulus {
            extern "C" {
                #[link_name = "cxxbridge1$Cmodulus_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __Cmodulus_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __Cmodulus_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn Cmodulus_free_autocxx_wrapper_0xe97f7296bea4f464(arg0: *mut Cmodulus) {
            extern "C" {
                #[link_name = "cxxbridge1$Cmodulus_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __Cmodulus_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __Cmodulus_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        ///! Copy constructor
        pub unsafe fn helib_Cmodulus_new2_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: *mut Cmodulus,
            other: &Cmodulus,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_Cmodulus_new2_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_Cmodulus_new2_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *const ::cxx::core::ffi::c_void,
                );
            }
            __helib_Cmodulus_new2_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this.cast(),
                other as *const Cmodulus as *const ::cxx::core::ffi::c_void,
            )
        }
        pub unsafe fn ZZ_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut ZZ {
            extern "C" {
                #[link_name = "cxxbridge1$ZZ_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __ZZ_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut ::cxx::core::ffi::c_void;
            }
            __ZZ_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn ZZ_free_autocxx_wrapper_0xe97f7296bea4f464(arg0: *mut ZZ) {
            extern "C" {
                #[link_name = "cxxbridge1$ZZ_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __ZZ_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __ZZ_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        pub unsafe fn NTL_ZZ_new3_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: *mut ZZ,
            a: &ZZ,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$NTL_ZZ_new3_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __NTL_ZZ_new3_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    a: *const ::cxx::core::ffi::c_void,
                );
            }
            __NTL_ZZ_new3_autocxx_wrapper_0xe97f7296bea4f464(
                autocxx_gen_this.cast(),
                a as *const ZZ as *const ::cxx::core::ffi::c_void,
            )
        }
        pub unsafe fn new11_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: *mut ZZ,
            a: *mut ZZ,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$new11_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __new11_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    a: *mut ::cxx::core::ffi::c_void,
                );
            }
            __new11_autocxx_wrapper_0xe97f7296bea4f464(autocxx_gen_this.cast(), a.cast())
        }
        pub unsafe fn JsonWrapper_alloc_autocxx_wrapper_0xe97f7296bea4f464() -> *mut JsonWrapper {
            extern "C" {
                #[link_name = "cxxbridge1$JsonWrapper_alloc_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __JsonWrapper_alloc_autocxx_wrapper_0xe97f7296bea4f464(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __JsonWrapper_alloc_autocxx_wrapper_0xe97f7296bea4f464().cast()
        }
        pub unsafe fn JsonWrapper_free_autocxx_wrapper_0xe97f7296bea4f464(arg0: *mut JsonWrapper) {
            extern "C" {
                #[link_name = "cxxbridge1$JsonWrapper_free_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __JsonWrapper_free_autocxx_wrapper_0xe97f7296bea4f464(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __JsonWrapper_free_autocxx_wrapper_0xe97f7296bea4f464(arg0.cast())
        }
        ///Synthesized move constructor.
        pub unsafe fn helib_JsonWrapper_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464(
            autocxx_gen_this: *mut JsonWrapper,
            other: *mut JsonWrapper,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$helib_JsonWrapper_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464"]
                fn __helib_JsonWrapper_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    other: *mut ::cxx::core::ffi::c_void,
                );
            }
            __helib_JsonWrapper_new_synthetic_move_ctor_0xe97f7296bea4f464_autocxx_wrapper_0xe97f7296bea4f464 (autocxx_gen_this . cast () , other . cast ())
        }
        pub type c_long = autocxx::c_long;
        pub type c_ulong = autocxx::c_ulong;
        unsafe impl ::cxx::private::UniquePtrTarget for Context {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("Context")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Context$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Context$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<Context>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Context$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Context$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Context$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Context$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for Context {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("Context")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$Context$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$Context$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<Context>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$Context$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$Context$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$Context$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for Context {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("Context")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$Context$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$Context$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$Context$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$Context$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$Context$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for xdouble {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("xdouble")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$xdouble$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$xdouble$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<xdouble>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$xdouble$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$xdouble$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$xdouble$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$xdouble$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for xdouble {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("xdouble")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL$xdouble$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL$xdouble$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<xdouble>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL$xdouble$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL$xdouble$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL$xdouble$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for xdouble {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("xdouble")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL$xdouble$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL$xdouble$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL$xdouble$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL$xdouble$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL$xdouble$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for PowerfulDCRT {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PowerfulDCRT")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PowerfulDCRT$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PowerfulDCRT$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<PowerfulDCRT>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PowerfulDCRT$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PowerfulDCRT$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PowerfulDCRT$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PowerfulDCRT$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for PowerfulDCRT {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PowerfulDCRT")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PowerfulDCRT$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PowerfulDCRT$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<PowerfulDCRT>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PowerfulDCRT$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PowerfulDCRT$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PowerfulDCRT$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for PowerfulDCRT {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PowerfulDCRT")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PowerfulDCRT$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PowerfulDCRT$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PowerfulDCRT$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PowerfulDCRT$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PowerfulDCRT$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for PolyModRing {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PolyModRing")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PolyModRing$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PolyModRing$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<PolyModRing>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PolyModRing$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PolyModRing$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PolyModRing$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PolyModRing$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for PolyModRing {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PolyModRing")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PolyModRing$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PolyModRing$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<PolyModRing>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PolyModRing$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PolyModRing$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PolyModRing$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for PolyModRing {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PolyModRing")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PolyModRing$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PolyModRing$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PolyModRing$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PolyModRing$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PolyModRing$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for PolyModRing {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PolyModRing")
            }
            fn __vector_new() -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$PolyModRing$new"]
                    fn __vector_new() -> *mut ::cxx::CxxVector<PolyModRing>;
                }
                unsafe { __vector_new() }
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$PolyModRing$size"]
                    fn __vector_size(_: &::cxx::CxxVector<PolyModRing>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(v: *mut ::cxx::CxxVector<Self>, pos: usize) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$PolyModRing$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<PolyModRing>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$PolyModRing$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<PolyModRing>>,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$PolyModRing$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<PolyModRing>>,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$PolyModRing$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$PolyModRing$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::CxxVector<PolyModRing>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$PolyModRing$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::CxxVector<PolyModRing>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$PolyModRing$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::CxxVector<PolyModRing>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$PolyModRing$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for IndexSet {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("IndexSet")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$IndexSet$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$IndexSet$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<IndexSet>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$IndexSet$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$IndexSet$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$IndexSet$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$IndexSet$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for IndexSet {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("IndexSet")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$IndexSet$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$IndexSet$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<IndexSet>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$IndexSet$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$IndexSet$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$IndexSet$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for IndexSet {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("IndexSet")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$IndexSet$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$IndexSet$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$IndexSet$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$IndexSet$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$IndexSet$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for IndexSet {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("IndexSet")
            }
            fn __vector_new() -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$IndexSet$new"]
                    fn __vector_new() -> *mut ::cxx::CxxVector<IndexSet>;
                }
                unsafe { __vector_new() }
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$IndexSet$size"]
                    fn __vector_size(_: &::cxx::CxxVector<IndexSet>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(v: *mut ::cxx::CxxVector<Self>, pos: usize) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$IndexSet$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<IndexSet>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$IndexSet$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<IndexSet>>,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$IndexSet$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<IndexSet>>,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$IndexSet$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$IndexSet$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::CxxVector<IndexSet>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$IndexSet$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::CxxVector<IndexSet>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$IndexSet$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::CxxVector<IndexSet>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$IndexSet$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for ThinRecryptData {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ThinRecryptData")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ThinRecryptData$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ThinRecryptData$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<ThinRecryptData>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ThinRecryptData$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ThinRecryptData$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ThinRecryptData$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ThinRecryptData$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for ThinRecryptData {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ThinRecryptData")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ThinRecryptData$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ThinRecryptData$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<ThinRecryptData>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ThinRecryptData$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ThinRecryptData$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ThinRecryptData$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for ThinRecryptData {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ThinRecryptData")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ThinRecryptData$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ThinRecryptData$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ThinRecryptData$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ThinRecryptData$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ThinRecryptData$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for PAlgebra {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PAlgebra")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebra$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebra$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<PAlgebra>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebra$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebra$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebra$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebra$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for PAlgebra {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PAlgebra")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PAlgebra$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PAlgebra$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<PAlgebra>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PAlgebra$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PAlgebra$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PAlgebra$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for PAlgebra {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PAlgebra")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PAlgebra$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PAlgebra$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PAlgebra$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PAlgebra$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PAlgebra$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for PAlgebraMod {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PAlgebraMod")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebraMod$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebraMod$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<PAlgebraMod>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebraMod$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebraMod$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebraMod$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$PAlgebraMod$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for PAlgebraMod {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PAlgebraMod")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PAlgebraMod$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PAlgebraMod$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<PAlgebraMod>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PAlgebraMod$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PAlgebraMod$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$PAlgebraMod$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for PAlgebraMod {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("PAlgebraMod")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PAlgebraMod$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PAlgebraMod$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PAlgebraMod$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PAlgebraMod$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$PAlgebraMod$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for EncryptedArray {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("EncryptedArray")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$EncryptedArray$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$EncryptedArray$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<EncryptedArray>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$EncryptedArray$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$EncryptedArray$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$EncryptedArray$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$EncryptedArray$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for EncryptedArray {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("EncryptedArray")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$EncryptedArray$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$EncryptedArray$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<EncryptedArray>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$EncryptedArray$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$EncryptedArray$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$EncryptedArray$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for EncryptedArray {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("EncryptedArray")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$EncryptedArray$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$EncryptedArray$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$EncryptedArray$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$EncryptedArray$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$EncryptedArray$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for ModuliSizes {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ModuliSizes")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ModuliSizes$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ModuliSizes$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<ModuliSizes>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ModuliSizes$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ModuliSizes$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ModuliSizes$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ModuliSizes$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for ModuliSizes {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ModuliSizes")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ModuliSizes$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ModuliSizes$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<ModuliSizes>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ModuliSizes$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ModuliSizes$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ModuliSizes$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for ModuliSizes {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ModuliSizes")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ModuliSizes$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ModuliSizes$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ModuliSizes$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ModuliSizes$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ModuliSizes$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for ModuliSizes {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ModuliSizes")
            }
            fn __vector_new() -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$ModuliSizes$new"]
                    fn __vector_new() -> *mut ::cxx::CxxVector<ModuliSizes>;
                }
                unsafe { __vector_new() }
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$ModuliSizes$size"]
                    fn __vector_size(_: &::cxx::CxxVector<ModuliSizes>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(v: *mut ::cxx::CxxVector<Self>, pos: usize) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$ModuliSizes$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<ModuliSizes>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$ModuliSizes$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<ModuliSizes>>,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$ModuliSizes$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<ModuliSizes>>,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$ModuliSizes$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$ModuliSizes$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::CxxVector<ModuliSizes>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$ModuliSizes$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::CxxVector<ModuliSizes>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$ModuliSizes$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::CxxVector<ModuliSizes>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$ModuliSizes$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for NTL_Vec_long_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("NTL_Vec_long_AutocxxConcrete")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL_Vec_long_AutocxxConcrete$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL_Vec_long_AutocxxConcrete$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL_Vec_long_AutocxxConcrete$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL_Vec_long_AutocxxConcrete$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL_Vec_long_AutocxxConcrete$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for NTL_Vec_long_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("NTL_Vec_long_AutocxxConcrete")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL_Vec_long_AutocxxConcrete$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL_Vec_long_AutocxxConcrete$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL_Vec_long_AutocxxConcrete$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL_Vec_long_AutocxxConcrete$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for NTL_Vec_long_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("NTL_Vec_long_AutocxxConcrete")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL_Vec_long_AutocxxConcrete$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL_Vec_long_AutocxxConcrete$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL_Vec_long_AutocxxConcrete$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL_Vec_long_AutocxxConcrete$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL_Vec_long_AutocxxConcrete$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for Cmodulus {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("Cmodulus")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Cmodulus$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Cmodulus$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<Cmodulus>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Cmodulus$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Cmodulus$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Cmodulus$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$Cmodulus$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for Cmodulus {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("Cmodulus")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$Cmodulus$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$Cmodulus$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<Cmodulus>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$Cmodulus$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$Cmodulus$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$Cmodulus$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for Cmodulus {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("Cmodulus")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$Cmodulus$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$Cmodulus$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$Cmodulus$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$Cmodulus$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$Cmodulus$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for ZZ {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ZZ")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$ZZ$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$ZZ$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<ZZ>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$ZZ$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$ZZ$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$ZZ$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$NTL$ZZ$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for ZZ {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ZZ")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL$ZZ$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL$ZZ$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<ZZ>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL$ZZ$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL$ZZ$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$NTL$ZZ$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for ZZ {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ZZ")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL$ZZ$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL$ZZ$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL$ZZ$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL$ZZ$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$NTL$ZZ$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for ZZ {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ZZ")
            }
            fn __vector_new() -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$NTL$ZZ$new"]
                    fn __vector_new() -> *mut ::cxx::CxxVector<ZZ>;
                }
                unsafe { __vector_new() }
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$NTL$ZZ$size"]
                    fn __vector_size(_: &::cxx::CxxVector<ZZ>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(v: *mut ::cxx::CxxVector<Self>, pos: usize) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$NTL$ZZ$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<ZZ>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$NTL$ZZ$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<ZZ>>,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$NTL$ZZ$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<ZZ>>,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$NTL$ZZ$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$NTL$ZZ$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::CxxVector<ZZ>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$NTL$ZZ$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::CxxVector<ZZ>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$NTL$ZZ$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::CxxVector<ZZ>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$NTL$ZZ$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for std_basic_ostream_char_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("std_basic_ostream_char_AutocxxConcrete")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std_basic_ostream_char_AutocxxConcrete$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std_basic_ostream_char_AutocxxConcrete$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std_basic_ostream_char_AutocxxConcrete$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std_basic_ostream_char_AutocxxConcrete$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std_basic_ostream_char_AutocxxConcrete$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for std_basic_ostream_char_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("std_basic_ostream_char_AutocxxConcrete")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_basic_ostream_char_AutocxxConcrete$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_basic_ostream_char_AutocxxConcrete$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_basic_ostream_char_AutocxxConcrete$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_basic_ostream_char_AutocxxConcrete$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for std_basic_ostream_char_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("std_basic_ostream_char_AutocxxConcrete")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_basic_ostream_char_AutocxxConcrete$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_basic_ostream_char_AutocxxConcrete$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_basic_ostream_char_AutocxxConcrete$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_basic_ostream_char_AutocxxConcrete$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_basic_ostream_char_AutocxxConcrete$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for std_basic_istream_char_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("std_basic_istream_char_AutocxxConcrete")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std_basic_istream_char_AutocxxConcrete$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std_basic_istream_char_AutocxxConcrete$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std_basic_istream_char_AutocxxConcrete$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std_basic_istream_char_AutocxxConcrete$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std_basic_istream_char_AutocxxConcrete$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for std_basic_istream_char_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("std_basic_istream_char_AutocxxConcrete")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_basic_istream_char_AutocxxConcrete$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_basic_istream_char_AutocxxConcrete$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_basic_istream_char_AutocxxConcrete$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_basic_istream_char_AutocxxConcrete$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for std_basic_istream_char_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("std_basic_istream_char_AutocxxConcrete")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_basic_istream_char_AutocxxConcrete$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_basic_istream_char_AutocxxConcrete$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_basic_istream_char_AutocxxConcrete$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_basic_istream_char_AutocxxConcrete$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_basic_istream_char_AutocxxConcrete$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for JsonWrapper {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("JsonWrapper")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$JsonWrapper$null"]
                    fn __null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __null(&mut repr) }
                repr
            }
            fn __new(value: Self) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$JsonWrapper$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<JsonWrapper>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$JsonWrapper$raw"]
                    fn __raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::core::ffi::c_void,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __raw(&mut repr, raw.cast());
                repr
            }
            unsafe fn __get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$JsonWrapper$get"]
                    fn __get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(&repr).cast()
            }
            unsafe fn __release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$JsonWrapper$release"]
                    fn __release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __release(&mut repr).cast()
            }
            unsafe fn __drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$JsonWrapper$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for JsonWrapper {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("JsonWrapper")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$JsonWrapper$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$JsonWrapper$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<JsonWrapper>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$JsonWrapper$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$JsonWrapper$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$JsonWrapper$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for JsonWrapper {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("JsonWrapper")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$JsonWrapper$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$JsonWrapper$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __downgrade(
                shared: *const ::cxx::core::ffi::c_void,
                weak: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$JsonWrapper$downgrade"]
                    fn __downgrade(
                        shared: *const ::cxx::core::ffi::c_void,
                        weak: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __downgrade(shared, weak);
            }
            unsafe fn __upgrade(
                weak: *const ::cxx::core::ffi::c_void,
                shared: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$JsonWrapper$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$JsonWrapper$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for JsonWrapper {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("JsonWrapper")
            }
            fn __vector_new() -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$JsonWrapper$new"]
                    fn __vector_new() -> *mut ::cxx::CxxVector<JsonWrapper>;
                }
                unsafe { __vector_new() }
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$JsonWrapper$size"]
                    fn __vector_size(_: &::cxx::CxxVector<JsonWrapper>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(v: *mut ::cxx::CxxVector<Self>, pos: usize) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$JsonWrapper$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<JsonWrapper>,
                        pos: usize,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __get_unchecked(v, pos) as *mut Self
            }
            unsafe fn __push_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                value: &mut ::cxx::core::mem::ManuallyDrop<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$JsonWrapper$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<JsonWrapper>>,
                        value: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __push_back(
                    this,
                    value as *mut ::cxx::core::mem::ManuallyDrop<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            unsafe fn __pop_back(
                this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<Self>>,
                out: &mut ::cxx::core::mem::MaybeUninit<Self>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$JsonWrapper$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<JsonWrapper>>,
                        out: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __pop_back(
                    this,
                    out as *mut ::cxx::core::mem::MaybeUninit<Self>
                        as *mut ::cxx::core::ffi::c_void,
                );
            }
            fn __unique_ptr_null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$JsonWrapper$null"]
                    fn __unique_ptr_null(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __unique_ptr_null(&mut repr) }
                repr
            }
            unsafe fn __unique_ptr_raw(
                raw: *mut ::cxx::CxxVector<Self>,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$JsonWrapper$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::CxxVector<JsonWrapper>,
                    );
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                __unique_ptr_raw(&mut repr, raw);
                repr
            }
            unsafe fn __unique_ptr_get(
                repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *const ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$JsonWrapper$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::CxxVector<JsonWrapper>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$JsonWrapper$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::CxxVector<JsonWrapper>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$JsonWrapper$drop"]
                    fn __unique_ptr_drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __unique_ptr_drop(&mut repr);
            }
        }
        #[doc(hidden)]
        const _: () = {
            const _: fn() = ::cxx::private::verify_extern_type::<
                Context,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::C,
                    ::cxx::o,
                    ::cxx::n,
                    ::cxx::t,
                    ::cxx::e,
                    ::cxx::x,
                    ::cxx::t,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                xdouble,
                (
                    ::cxx::N,
                    ::cxx::T,
                    ::cxx::L,
                    (),
                    ::cxx::x,
                    ::cxx::d,
                    ::cxx::o,
                    ::cxx::u,
                    ::cxx::b,
                    ::cxx::l,
                    ::cxx::e,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                PowerfulDCRT,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::P,
                    ::cxx::o,
                    ::cxx::w,
                    ::cxx::e,
                    ::cxx::r,
                    ::cxx::f,
                    ::cxx::u,
                    ::cxx::l,
                    ::cxx::D,
                    ::cxx::C,
                    ::cxx::R,
                    ::cxx::T,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                PolyModRing,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::P,
                    ::cxx::o,
                    ::cxx::l,
                    ::cxx::y,
                    ::cxx::M,
                    ::cxx::o,
                    ::cxx::d,
                    ::cxx::R,
                    ::cxx::i,
                    ::cxx::n,
                    ::cxx::g,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                IndexSet,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::I,
                    ::cxx::n,
                    ::cxx::d,
                    ::cxx::e,
                    ::cxx::x,
                    ::cxx::S,
                    ::cxx::e,
                    ::cxx::t,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                ThinRecryptData,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::T,
                    ::cxx::h,
                    ::cxx::i,
                    ::cxx::n,
                    ::cxx::R,
                    ::cxx::e,
                    ::cxx::c,
                    ::cxx::r,
                    ::cxx::y,
                    ::cxx::p,
                    ::cxx::t,
                    ::cxx::D,
                    ::cxx::a,
                    ::cxx::t,
                    ::cxx::a,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                PAlgebra,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::P,
                    ::cxx::A,
                    ::cxx::l,
                    ::cxx::g,
                    ::cxx::e,
                    ::cxx::b,
                    ::cxx::r,
                    ::cxx::a,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                PAlgebraMod,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::P,
                    ::cxx::A,
                    ::cxx::l,
                    ::cxx::g,
                    ::cxx::e,
                    ::cxx::b,
                    ::cxx::r,
                    ::cxx::a,
                    ::cxx::M,
                    ::cxx::o,
                    ::cxx::d,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                EncryptedArray,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::E,
                    ::cxx::n,
                    ::cxx::c,
                    ::cxx::r,
                    ::cxx::y,
                    ::cxx::p,
                    ::cxx::t,
                    ::cxx::e,
                    ::cxx::d,
                    ::cxx::A,
                    ::cxx::r,
                    ::cxx::r,
                    ::cxx::a,
                    ::cxx::y,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                ModuliSizes,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::M,
                    ::cxx::o,
                    ::cxx::d,
                    ::cxx::u,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::S,
                    ::cxx::i,
                    ::cxx::z,
                    ::cxx::e,
                    ::cxx::s,
                ),
            >;
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                impl<T> __AmbiguousIfImpl<()> for T where T: ?::cxx::core::marker::Sized {}
                #[allow(dead_code)]
                struct __Invalid;
                impl<T> __AmbiguousIfImpl<__Invalid> for T where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin
                {
                }
                <NTL_Vec_long_AutocxxConcrete as __AmbiguousIfImpl<_>>::infer
            };
            const _: fn() = ::cxx::private::verify_extern_type::<
                Cmodulus,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::C,
                    ::cxx::m,
                    ::cxx::o,
                    ::cxx::d,
                    ::cxx::u,
                    ::cxx::l,
                    ::cxx::u,
                    ::cxx::s,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                ZZ,
                (::cxx::N, ::cxx::T, ::cxx::L, (), ::cxx::Z, ::cxx::Z),
            >;
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                impl<T> __AmbiguousIfImpl<()> for T where T: ?::cxx::core::marker::Sized {}
                #[allow(dead_code)]
                struct __Invalid;
                impl<T> __AmbiguousIfImpl<__Invalid> for T where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin
                {
                }
                <std_basic_ostream_char_AutocxxConcrete as __AmbiguousIfImpl<_>>::infer
            };
            let _: fn() = {
                trait __AmbiguousIfImpl<A> {
                    fn infer() {}
                }
                impl<T> __AmbiguousIfImpl<()> for T where T: ?::cxx::core::marker::Sized {}
                #[allow(dead_code)]
                struct __Invalid;
                impl<T> __AmbiguousIfImpl<__Invalid> for T where
                    T: ?::cxx::core::marker::Sized + ::cxx::core::marker::Unpin
                {
                }
                <std_basic_istream_char_AutocxxConcrete as __AmbiguousIfImpl<_>>::infer
            };
            const _: fn() = ::cxx::private::verify_extern_type::<
                JsonWrapper,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::J,
                    ::cxx::s,
                    ::cxx::o,
                    ::cxx::n,
                    ::cxx::W,
                    ::cxx::r,
                    ::cxx::a,
                    ::cxx::p,
                    ::cxx::p,
                    ::cxx::e,
                    ::cxx::r,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_type::<
                c_long,
                (::cxx::c, ::cxx::__, ::cxx::l, ::cxx::o, ::cxx::n, ::cxx::g),
            >;
            const _: fn() = ::cxx::private::verify_extern_kind::<c_long, ::cxx::kind::Trivial>;
            const _: fn() = ::cxx::private::verify_extern_type::<
                c_ulong,
                (
                    ::cxx::c,
                    ::cxx::__,
                    ::cxx::u,
                    ::cxx::l,
                    ::cxx::o,
                    ::cxx::n,
                    ::cxx::g,
                ),
            >;
            const _: fn() = ::cxx::private::verify_extern_kind::<c_ulong, ::cxx::kind::Trivial>;
        };
    }
    #[allow(unused_imports)]
    use bindgen::root;
    pub use cxxbridge::autocxx_make_string_0xe97f7296bea4f464 as make_string;
    pub use bindgen::root::NTL_Vec_long_AutocxxConcrete;
    pub use bindgen::root::std_basic_ostream_char_AutocxxConcrete;
    pub use bindgen::root::std_basic_istream_char_AutocxxConcrete;
    pub mod NTL {
        pub use super::bindgen::root::NTL::xdouble;
        pub use super::bindgen::root::NTL::ZZ;
    }
    pub mod helib {
        pub use super::bindgen::root::helib::Context;
        pub use super::bindgen::root::helib::PowerfulDCRT;
        pub use super::bindgen::root::helib::PolyModRing;
        pub use super::bindgen::root::helib::IndexSet;
        pub use super::bindgen::root::helib::ThinRecryptData;
        pub use super::bindgen::root::helib::PAlgebra;
        pub use super::bindgen::root::helib::PAlgebraMod;
        pub use super::bindgen::root::helib::EncryptedArray;
        pub use super::bindgen::root::helib::ModuliSizes;
        pub use super::bindgen::root::helib::Cmodulus;
        pub use super::bindgen::root::helib::JsonWrapper;
    }
    pub mod std {
        pub use super::bindgen::root::std::ostream;
        pub use super::bindgen::root::std::istream;
    }
// }
