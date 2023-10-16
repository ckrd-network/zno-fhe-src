#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
pub mod prelude {}
mod zno {
    mod types {}
    mod helib {
        mod ffi {
            pub mod bgv {}
        }
    }
}
use autocxx::prelude::*;
#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
mod ffi {
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
    unsafe impl cxx::ExternType for bindgen::root::helib::ArgMap {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::A,
            ::cxx::r,
            ::cxx::g,
            ::cxx::M,
            ::cxx::a,
            ::cxx::p,
        );
        type Kind = cxx::kind::Opaque;
    }
    unsafe impl cxx::ExternType for bindgen::root::helib::ArgMap_Separator {
        type Id = (
            ::cxx::h,
            ::cxx::e,
            ::cxx::l,
            ::cxx::i,
            ::cxx::b,
            (),
            ::cxx::A,
            ::cxx::r,
            ::cxx::g,
            ::cxx::M,
            ::cxx::a,
            ::cxx::p,
            (),
            ::cxx::S,
            ::cxx::e,
            ::cxx::p,
            ::cxx::a,
            ::cxx::r,
            ::cxx::a,
            ::cxx::t,
            ::cxx::o,
            ::cxx::r,
        );
        type Kind = cxx::kind::Trivial;
    }
    mod bindgen {
        pub(super) mod root {
            pub use cxxbridge::std_initializer_list_std_string_AutocxxConcrete;
            pub mod helib {
                /// @brief Basic class for arg parsing.
                /// Example use:
                /// @code
                ///   // Variables to be set by command line.
                ///   long p = 2;                                 // default values.
                ///   long m = 19;
                ///   bool t = false;
                ///   bool f = true;
                ///   std::string k = "Hello World";
                ///
                ///   ArgMap()                                    // (*) marks default.
                ///     .required()                               // set args to required.
                ///     .positional()                             //
                ///       .arg("p", p, "doc for p")               //
                ///       .arg("m", m, "doc for m", "undefined")  // special default info.
                ///     .optional()                               // swap to optional args (*).
                ///     .named()                                  // named args (*) e.g.k=v.
                ///     .separator(ArgMap::Separator::WHITESPACE) // change separator to
                ///       .arg("-k", k, "doc for k", "")          // whitespace ('=' is (*)).
                ///       .note("an extra note")                  // no default value info.
                ///     .toggle()                                 // add extra doc/note.
                ///        .arg("-t", t, "doc for t", "")         // toggle flag sets bool true.
                ///     .toggle(false)                            // toggle flag sets bool false.
                ///        .arg("-f", f, "doc for f", "")         //
                ///     .helpArgs({"--myhelp"})                   // changes default help flags
                ///     .parse(argc, argv);                       // (*) is {"-h", "--help"}.
                ///                                               // parses and overwrites values
                /// @endcode
                #[repr(C, align(8))]
                pub struct ArgMap {
                    _pinned: core::marker::PhantomData<core::marker::PhantomPinned>,
                    _non_send_sync: core::marker::PhantomData<[*const u8; 0]>,
                    _data: [u8; 440],
                }
                #[repr(i32)]
                pub enum ArgMap_Separator {
                    COLON = 0,
                    EQUALS = 1,
                    WHITESPACE = 2,
                }
                #[automatically_derived]
                impl ::core::clone::Clone for ArgMap_Separator {
                    #[inline]
                    fn clone(&self) -> ArgMap_Separator {
                        match self {
                            ArgMap_Separator::COLON => ArgMap_Separator::COLON,
                            ArgMap_Separator::EQUALS => ArgMap_Separator::EQUALS,
                            ArgMap_Separator::WHITESPACE => ArgMap_Separator::WHITESPACE,
                        }
                    }
                }
                #[automatically_derived]
                impl ::core::hash::Hash for ArgMap_Separator {
                    #[inline]
                    fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
                        let __self_tag = ::core::intrinsics::discriminant_value(self);
                        ::core::hash::Hash::hash(&__self_tag, state)
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralPartialEq for ArgMap_Separator {}
                #[automatically_derived]
                impl ::core::cmp::PartialEq for ArgMap_Separator {
                    #[inline]
                    fn eq(&self, other: &ArgMap_Separator) -> bool {
                        let __self_tag = ::core::intrinsics::discriminant_value(self);
                        let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                        __self_tag == __arg1_tag
                    }
                }
                #[automatically_derived]
                impl ::core::marker::StructuralEq for ArgMap_Separator {}
                #[automatically_derived]
                impl ::core::cmp::Eq for ArgMap_Separator {
                    #[inline]
                    #[doc(hidden)]
                    #[coverage(off)]
                    fn assert_receiver_is_total_eq(&self) -> () {}
                }
                impl ArgMap {
                    ///autocxx bindings couldn't be generated: This function or method uses a type where one of the template parameters was incomprehensible to bindgen/autocxx - probably because it uses template specialization.
                    fn printDiagnostics(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: This function or method uses a type where one of the template parameters was incomprehensible to bindgen/autocxx - probably because it uses template specialization.
                    fn simpleParse(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: Problem handling function argument argv: Pointer pointed to another pointer, which is not yet supported
                    fn parse(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: Function parse1 has a reference return value, but >1 input reference parameters, so the lifetime of the output reference cannot be deduced.
                    fn parse1(_uhoh: autocxx::BindingGenerationFailure) {}
                    /// @brief Provide custom help toggle args. (defaults are "-h", "--help")
                    /// Overwrite default help toggle args to custom ones for parsing.
                    /// @return A reference to the ArgMap object
                    pub fn helpArgs<'a>(
                        self: ::core::pin::Pin<&'a mut root::helib::ArgMap>,
                        s: impl autocxx::ValueParam<
                                root::std_initializer_list_std_string_AutocxxConcrete,
                            > + 'a,
                    ) -> ::core::pin::Pin<&'a mut root::helib::ArgMap> {
                        let mut space0 = autocxx::ValueParamHandler::default();
                        let mut space0 = unsafe { ::core::pin::Pin::new_unchecked(&mut space0) };
                        unsafe {
                            space0.as_mut().populate(s);
                            cxxbridge::helpArgs_autocxx_wrapper_0xfdd77103f48d7970(
                                self,
                                space0.get_ptr(),
                            )
                        }
                    }
                    pub fn helpArgs1<'a>(
                        self: ::core::pin::Pin<&'a mut root::helib::ArgMap>,
                        s: impl ToCppString + 'a,
                    ) -> ::core::pin::Pin<&'a mut root::helib::ArgMap> {
                        cxxbridge::helpArgs1_autocxx_wrapper_0xfdd77103f48d7970(self, s.into_cpp())
                    }
                    ///autocxx bindings couldn't be generated: Function diagnostics has a mutable reference return value, but >1 input mutable reference parameters, so the lifetime of the output reference cannot be deduced.
                    fn diagnostics(_uhoh: autocxx::BindingGenerationFailure) {}
                    ///autocxx bindings couldn't be generated: Function note has a reference return value, but >1 input reference parameters, so the lifetime of the output reference cannot be deduced.
                    fn note(_uhoh: autocxx::BindingGenerationFailure) {}
                    /// @brief Return arg docs
                    /// Returns the argument documentation as a string
                    /// @return the argument documentation string
                    pub fn doc(self: &root::helib::ArgMap) -> cxx::UniquePtr<cxx::CxxString> {
                        cxxbridge::doc_autocxx_wrapper_0xfdd77103f48d7970(self)
                    }
                }
                unsafe impl autocxx::moveit::MakeCppStorage for root::helib::ArgMap {
                    unsafe fn allocate_uninitialized_cpp_storage() -> *mut root::helib::ArgMap {
                        cxxbridge::ArgMap_alloc_autocxx_wrapper_0xfdd77103f48d7970()
                    }
                    unsafe fn free_uninitialized_cpp_storage(arg0: *mut root::helib::ArgMap) {
                        cxxbridge::ArgMap_free_autocxx_wrapper_0xfdd77103f48d7970(arg0)
                    }
                }
                impl Drop for root::helib::ArgMap {
                    ///Synthesized destructor.
                    fn drop(self: &mut root::helib::ArgMap) {
                        unsafe {
                            cxxbridge :: ArgMap_synthetic_destructor_0xfdd77103f48d7970_autocxx_wrapper_0xfdd77103f48d7970 (self)
                        }
                    }
                }
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
        pub fn autocxx_make_string_0xfdd77103f48d7970(
            str_: &str,
        ) -> ::cxx::UniquePtr<::cxx::CxxString> {
            extern "C" {
                #[link_name = "cxxbridge1$autocxx_make_string_0xfdd77103f48d7970"]
                fn __autocxx_make_string_0xfdd77103f48d7970(
                    str_: ::cxx::private::RustStr,
                ) -> *mut ::cxx::CxxString;
            }
            unsafe {
                ::cxx::UniquePtr::from_raw(__autocxx_make_string_0xfdd77103f48d7970(
                    ::cxx::private::RustStr::from(str_),
                ))
            }
        }
        pub unsafe fn ArgMap_alloc_autocxx_wrapper_0xfdd77103f48d7970() -> *mut ArgMap {
            extern "C" {
                #[link_name = "cxxbridge1$ArgMap_alloc_autocxx_wrapper_0xfdd77103f48d7970"]
                fn __ArgMap_alloc_autocxx_wrapper_0xfdd77103f48d7970(
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            __ArgMap_alloc_autocxx_wrapper_0xfdd77103f48d7970().cast()
        }
        pub unsafe fn ArgMap_free_autocxx_wrapper_0xfdd77103f48d7970(arg0: *mut ArgMap) {
            extern "C" {
                #[link_name = "cxxbridge1$ArgMap_free_autocxx_wrapper_0xfdd77103f48d7970"]
                fn __ArgMap_free_autocxx_wrapper_0xfdd77103f48d7970(
                    arg0: *mut ::cxx::core::ffi::c_void,
                );
            }
            __ArgMap_free_autocxx_wrapper_0xfdd77103f48d7970(arg0.cast())
        }
        /// @brief Basic class for arg parsing.
        /// Example use:
        /// @code
        ///   // Variables to be set by command line.
        ///   long p = 2;                                 // default values.
        ///   long m = 19;
        ///   bool t = false;
        ///   bool f = true;
        ///   std::string k = "Hello World";
        ///
        ///   ArgMap()                                    // (*) marks default.
        ///     .required()                               // set args to required.
        ///     .positional()                             //
        ///       .arg("p", p, "doc for p")               //
        ///       .arg("m", m, "doc for m", "undefined")  // special default info.
        ///     .optional()                               // swap to optional args (*).
        ///     .named()                                  // named args (*) e.g.k=v.
        ///     .separator(ArgMap::Separator::WHITESPACE) // change separator to
        ///       .arg("-k", k, "doc for k", "")          // whitespace ('=' is (*)).
        ///       .note("an extra note")                  // no default value info.
        ///     .toggle()                                 // add extra doc/note.
        ///        .arg("-t", t, "doc for t", "")         // toggle flag sets bool true.
        ///     .toggle(false)                            // toggle flag sets bool false.
        ///        .arg("-f", f, "doc for f", "")         //
        ///     .helpArgs({"--myhelp"})                   // changes default help flags
        ///     .parse(argc, argv);                       // (*) is {"-h", "--help"}.
        ///                                               // parses and overwrites values
        /// @endcode
        pub type ArgMap = super::bindgen::root::helib::ArgMap;
        impl<'a> ArgMap {
            /// @brief Swaps to optional arg mode (default)
            /// Swaps to optional arg mode. Following arguments will be considered optional
            /// @return A reference to the ArgMap object
            pub fn optional(
                self: ::cxx::core::pin::Pin<&'a mut Self>,
            ) -> ::cxx::core::pin::Pin<&'a mut ArgMap> {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$ArgMap$optional"]
                    fn __optional<'a>(
                        _: ::cxx::core::pin::Pin<&'a mut ArgMap>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { ::cxx::core::pin::Pin::new_unchecked(&mut *__optional(self).cast()) }
            }
        }
        impl<'a> ArgMap {
            /// @brief Swaps to required arg mode
            /// Swaps to required arg mode. Following arguments will be considered required
            /// @return A reference to the ArgMap object
            pub fn required(
                self: ::cxx::core::pin::Pin<&'a mut Self>,
            ) -> ::cxx::core::pin::Pin<&'a mut ArgMap> {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$ArgMap$required"]
                    fn __required<'a>(
                        _: ::cxx::core::pin::Pin<&'a mut ArgMap>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { ::cxx::core::pin::Pin::new_unchecked(&mut *__required(self).cast()) }
            }
        }
        impl<'a> ArgMap {
            /// @brief Swaps to toggle arg type
            /// Swaps to required arg mode. Following arguments will be considered of
            /// toggle type
            /// @return A reference to the ArgMap object
            pub fn toggle(
                self: ::cxx::core::pin::Pin<&'a mut Self>,
                t: bool,
            ) -> ::cxx::core::pin::Pin<&'a mut ArgMap> {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$ArgMap$toggle"]
                    fn __toggle<'a>(
                        _: ::cxx::core::pin::Pin<&'a mut ArgMap>,
                        t: bool,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { ::cxx::core::pin::Pin::new_unchecked(&mut *__toggle(self, t).cast()) }
            }
        }
        impl<'a> ArgMap {
            /// @brief Swaps to named arg type (default)
            /// Swaps to required arg mode. Following arguments will be considered of named
            /// type
            /// @return A reference to the ArgMap object
            pub fn named(
                self: ::cxx::core::pin::Pin<&'a mut Self>,
            ) -> ::cxx::core::pin::Pin<&'a mut ArgMap> {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$ArgMap$named"]
                    fn __named<'a>(
                        _: ::cxx::core::pin::Pin<&'a mut ArgMap>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { ::cxx::core::pin::Pin::new_unchecked(&mut *__named(self).cast()) }
            }
        }
        impl<'a> ArgMap {
            /// @brief Swaps to positional arg type
            /// Swaps to required arg mode. Following arguments will be considered of
            /// positional type
            /// @return A reference to the ArgMap object
            pub fn positional(
                self: ::cxx::core::pin::Pin<&'a mut Self>,
            ) -> ::cxx::core::pin::Pin<&'a mut ArgMap> {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$ArgMap$positional"]
                    fn __positional<'a>(
                        _: ::cxx::core::pin::Pin<&'a mut ArgMap>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe { ::cxx::core::pin::Pin::new_unchecked(&mut *__positional(self).cast()) }
            }
        }
        /// @brief Provide custom help toggle args. (defaults are "-h", "--help")
        /// Overwrite default help toggle args to custom ones for parsing.
        /// @return A reference to the ArgMap object
        pub unsafe fn helpArgs_autocxx_wrapper_0xfdd77103f48d7970<'a>(
            autocxx_gen_this: ::cxx::core::pin::Pin<&'a mut ArgMap>,
            s: *mut std_initializer_list_std_string_AutocxxConcrete,
        ) -> ::cxx::core::pin::Pin<&'a mut ArgMap> {
            extern "C" {
                #[link_name = "cxxbridge1$helpArgs_autocxx_wrapper_0xfdd77103f48d7970"]
                fn __helpArgs_autocxx_wrapper_0xfdd77103f48d7970<'a>(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    s: *mut std_initializer_list_std_string_AutocxxConcrete,
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            ::cxx::core::pin::Pin::new_unchecked(
                &mut *__helpArgs_autocxx_wrapper_0xfdd77103f48d7970(
                    ::cxx::core::pin::Pin::into_inner_unchecked(autocxx_gen_this) as *mut ArgMap
                        as *mut ::cxx::core::ffi::c_void,
                    s,
                )
                .cast(),
            )
        }
        pub fn helpArgs1_autocxx_wrapper_0xfdd77103f48d7970<'a>(
            autocxx_gen_this: ::cxx::core::pin::Pin<&'a mut ArgMap>,
            s: ::cxx::UniquePtr<::cxx::CxxString>,
        ) -> ::cxx::core::pin::Pin<&'a mut ArgMap> {
            extern "C" {
                #[link_name = "cxxbridge1$helpArgs1_autocxx_wrapper_0xfdd77103f48d7970"]
                fn __helpArgs1_autocxx_wrapper_0xfdd77103f48d7970<'a>(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                    s: *mut ::cxx::CxxString,
                ) -> *mut ::cxx::core::ffi::c_void;
            }
            unsafe {
                ::cxx::core::pin::Pin::new_unchecked(
                    &mut *__helpArgs1_autocxx_wrapper_0xfdd77103f48d7970(
                        ::cxx::core::pin::Pin::into_inner_unchecked(autocxx_gen_this) as *mut ArgMap
                            as *mut ::cxx::core::ffi::c_void,
                        ::cxx::UniquePtr::into_raw(s),
                    )
                    .cast(),
                )
            }
        }
        impl<'a> ArgMap {
            /// @brief Sets the key-value separator
            /// Sets the named args key-value pair separator character
            /// @param s the separator enum must be set either to COLON or EQUALS(default).
            /// @return A reference to the ArgMap object
            pub fn separator(
                self: ::cxx::core::pin::Pin<&'a mut Self>,
                s: ArgMap_Separator,
            ) -> ::cxx::core::pin::Pin<&'a mut ArgMap> {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$ArgMap$separator"]
                    fn __separator<'a>(
                        _: ::cxx::core::pin::Pin<&'a mut ArgMap>,
                        s: *mut ArgMap_Separator,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                unsafe {
                    let mut s = ::cxx::core::mem::MaybeUninit::new(s);
                    ::cxx::core::pin::Pin::new_unchecked(
                        &mut *__separator(self, s.as_mut_ptr()).cast(),
                    )
                }
            }
        }
        impl ArgMap {
            /// @brief Print usage and exit
            /// Prints the usage and exits the program
            /// @param msg An additional message to print before showing usage
            pub fn usage(&self, msg: &::cxx::CxxString) {
                extern "C" {
                    #[link_name = "helib$cxxbridge1$ArgMap$usage"]
                    fn __usage(_: &ArgMap, msg: &::cxx::CxxString);
                }
                unsafe { __usage(self, msg) }
            }
        }
        /// @brief Return arg docs
        /// Returns the argument documentation as a string
        /// @return the argument documentation string
        pub fn doc_autocxx_wrapper_0xfdd77103f48d7970(
            autocxx_gen_this: &ArgMap,
        ) -> ::cxx::UniquePtr<::cxx::CxxString> {
            extern "C" {
                #[link_name = "cxxbridge1$doc_autocxx_wrapper_0xfdd77103f48d7970"]
                fn __doc_autocxx_wrapper_0xfdd77103f48d7970(
                    autocxx_gen_this: *const ::cxx::core::ffi::c_void,
                ) -> *mut ::cxx::CxxString;
            }
            unsafe {
                ::cxx::UniquePtr::from_raw(__doc_autocxx_wrapper_0xfdd77103f48d7970(
                    autocxx_gen_this as *const ArgMap as *const ::cxx::core::ffi::c_void,
                ))
            }
        }
        ///Synthesized destructor.
        pub unsafe fn ArgMap_synthetic_destructor_0xfdd77103f48d7970_autocxx_wrapper_0xfdd77103f48d7970(
            autocxx_gen_this: *mut ArgMap,
        ) {
            extern "C" {
                #[link_name = "cxxbridge1$ArgMap_synthetic_destructor_0xfdd77103f48d7970_autocxx_wrapper_0xfdd77103f48d7970"]
                fn __ArgMap_synthetic_destructor_0xfdd77103f48d7970_autocxx_wrapper_0xfdd77103f48d7970(
                    autocxx_gen_this: *mut ::cxx::core::ffi::c_void,
                );
            }
            __ArgMap_synthetic_destructor_0xfdd77103f48d7970_autocxx_wrapper_0xfdd77103f48d7970(
                autocxx_gen_this.cast(),
            )
        }
        #[repr(C)]
        pub struct std_initializer_list_std_string_AutocxxConcrete {
            _private: ::cxx::private::Opaque,
        }
        unsafe impl ::cxx::ExternType for std_initializer_list_std_string_AutocxxConcrete {
            #[allow(unused_attributes)]
            #[doc(hidden)]
            type Id = (
                ::cxx::s,
                ::cxx::t,
                ::cxx::d,
                ::cxx::__,
                ::cxx::i,
                ::cxx::n,
                ::cxx::i,
                ::cxx::t,
                ::cxx::i,
                ::cxx::a,
                ::cxx::l,
                ::cxx::i,
                ::cxx::z,
                ::cxx::e,
                ::cxx::r,
                ::cxx::__,
                ::cxx::l,
                ::cxx::i,
                ::cxx::s,
                ::cxx::t,
                ::cxx::__,
                ::cxx::s,
                ::cxx::t,
                ::cxx::d,
                ::cxx::__,
                ::cxx::s,
                ::cxx::t,
                ::cxx::r,
                ::cxx::i,
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
        pub type ArgMap_Separator = super::bindgen::root::helib::ArgMap_Separator;
        unsafe impl ::cxx::private::UniquePtrTarget for ArgMap {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ArgMap")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$null"]
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
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<ArgMap>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$raw"]
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
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$get"]
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
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$release"]
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
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for ArgMap {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ArgMap")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ArgMap$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ArgMap$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<ArgMap>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ArgMap$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ArgMap$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ArgMap$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for ArgMap {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ArgMap")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ArgMap$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ArgMap$clone"]
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
                    #[link_name = "cxxbridge1$weak_ptr$helib$ArgMap$downgrade"]
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
                    #[link_name = "cxxbridge1$weak_ptr$helib$ArgMap$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ArgMap$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for std_initializer_list_std_string_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("std_initializer_list_std_string_AutocxxConcrete")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std_initializer_list_std_string_AutocxxConcrete$null"]
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
                    #[link_name = "cxxbridge1$unique_ptr$std_initializer_list_std_string_AutocxxConcrete$raw"]
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
                    #[link_name = "cxxbridge1$unique_ptr$std_initializer_list_std_string_AutocxxConcrete$get"]
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
                    #[link_name = "cxxbridge1$unique_ptr$std_initializer_list_std_string_AutocxxConcrete$release"]
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
                    #[link_name = "cxxbridge1$unique_ptr$std_initializer_list_std_string_AutocxxConcrete$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for std_initializer_list_std_string_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("std_initializer_list_std_string_AutocxxConcrete")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_initializer_list_std_string_AutocxxConcrete$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_initializer_list_std_string_AutocxxConcrete$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_initializer_list_std_string_AutocxxConcrete$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$std_initializer_list_std_string_AutocxxConcrete$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for std_initializer_list_std_string_AutocxxConcrete {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("std_initializer_list_std_string_AutocxxConcrete")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_initializer_list_std_string_AutocxxConcrete$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_initializer_list_std_string_AutocxxConcrete$clone"]
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
                    #[link_name = "cxxbridge1$weak_ptr$std_initializer_list_std_string_AutocxxConcrete$downgrade"]
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
                    #[link_name = "cxxbridge1$weak_ptr$std_initializer_list_std_string_AutocxxConcrete$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$std_initializer_list_std_string_AutocxxConcrete$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::UniquePtrTarget for ArgMap_Separator {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ArgMap_Separator")
            }
            fn __null() -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$Separator$null"]
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
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$Separator$uninit"]
                    fn __uninit(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                let mut repr = ::cxx::core::mem::MaybeUninit::uninit();
                unsafe { __uninit(&mut repr).cast::<ArgMap_Separator>().write(value) }
                repr
            }
            unsafe fn __raw(
                raw: *mut Self,
            ) -> ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$Separator$raw"]
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
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$Separator$get"]
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
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$Separator$release"]
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
                    #[link_name = "cxxbridge1$unique_ptr$helib$ArgMap$Separator$drop"]
                    fn __drop(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    );
                }
                __drop(&mut repr);
            }
        }
        unsafe impl ::cxx::private::SharedPtrTarget for ArgMap_Separator {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ArgMap_Separator")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ArgMap$Separator$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __new(value: Self, new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ArgMap$Separator$uninit"]
                    fn __uninit(
                        new: *mut ::cxx::core::ffi::c_void,
                    ) -> *mut ::cxx::core::ffi::c_void;
                }
                __uninit(new).cast::<ArgMap_Separator>().write(value);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ArgMap$Separator$clone"]
                    fn __clone(
                        this: *const ::cxx::core::ffi::c_void,
                        new: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __clone(this, new);
            }
            unsafe fn __get(this: *const ::cxx::core::ffi::c_void) -> *const Self {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ArgMap$Separator$get"]
                    fn __get(
                        this: *const ::cxx::core::ffi::c_void,
                    ) -> *const ::cxx::core::ffi::c_void;
                }
                __get(this).cast()
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$shared_ptr$helib$ArgMap$Separator$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::WeakPtrTarget for ArgMap_Separator {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ArgMap_Separator")
            }
            unsafe fn __null(new: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ArgMap$Separator$null"]
                    fn __null(new: *mut ::cxx::core::ffi::c_void);
                }
                __null(new);
            }
            unsafe fn __clone(
                this: *const ::cxx::core::ffi::c_void,
                new: *mut ::cxx::core::ffi::c_void,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ArgMap$Separator$clone"]
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
                    #[link_name = "cxxbridge1$weak_ptr$helib$ArgMap$Separator$downgrade"]
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
                    #[link_name = "cxxbridge1$weak_ptr$helib$ArgMap$Separator$upgrade"]
                    fn __upgrade(
                        weak: *const ::cxx::core::ffi::c_void,
                        shared: *mut ::cxx::core::ffi::c_void,
                    );
                }
                __upgrade(weak, shared);
            }
            unsafe fn __drop(this: *mut ::cxx::core::ffi::c_void) {
                extern "C" {
                    #[link_name = "cxxbridge1$weak_ptr$helib$ArgMap$Separator$drop"]
                    fn __drop(this: *mut ::cxx::core::ffi::c_void);
                }
                __drop(this);
            }
        }
        unsafe impl ::cxx::private::VectorElement for ArgMap_Separator {
            fn __typename(f: &mut ::cxx::core::fmt::Formatter<'_>) -> ::cxx::core::fmt::Result {
                f.write_str("ArgMap_Separator")
            }
            fn __vector_new() -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$ArgMap$Separator$new"]
                    fn __vector_new() -> *mut ::cxx::CxxVector<ArgMap_Separator>;
                }
                unsafe { __vector_new() }
            }
            fn __vector_size(v: &::cxx::CxxVector<Self>) -> usize {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$ArgMap$Separator$size"]
                    fn __vector_size(_: &::cxx::CxxVector<ArgMap_Separator>) -> usize;
                }
                unsafe { __vector_size(v) }
            }
            unsafe fn __get_unchecked(v: *mut ::cxx::CxxVector<Self>, pos: usize) -> *mut Self {
                extern "C" {
                    #[link_name = "cxxbridge1$std$vector$helib$ArgMap$Separator$get_unchecked"]
                    fn __get_unchecked(
                        v: *mut ::cxx::CxxVector<ArgMap_Separator>,
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
                    #[link_name = "cxxbridge1$std$vector$helib$ArgMap$Separator$push_back"]
                    fn __push_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<ArgMap_Separator>>,
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
                    #[link_name = "cxxbridge1$std$vector$helib$ArgMap$Separator$pop_back"]
                    fn __pop_back(
                        this: ::cxx::core::pin::Pin<&mut ::cxx::CxxVector<ArgMap_Separator>>,
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
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$ArgMap$Separator$null"]
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
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$ArgMap$Separator$raw"]
                    fn __unique_ptr_raw(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                        raw: *mut ::cxx::CxxVector<ArgMap_Separator>,
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
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$ArgMap$Separator$get"]
                    fn __unique_ptr_get(
                        this: *const ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *const ::cxx::CxxVector<ArgMap_Separator>;
                }
                __unique_ptr_get(&repr)
            }
            unsafe fn __unique_ptr_release(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) -> *mut ::cxx::CxxVector<Self> {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$ArgMap$Separator$release"]
                    fn __unique_ptr_release(
                        this: *mut ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
                    ) -> *mut ::cxx::CxxVector<ArgMap_Separator>;
                }
                __unique_ptr_release(&mut repr)
            }
            unsafe fn __unique_ptr_drop(
                mut repr: ::cxx::core::mem::MaybeUninit<*mut ::cxx::core::ffi::c_void>,
            ) {
                extern "C" {
                    #[link_name = "cxxbridge1$unique_ptr$std$vector$helib$ArgMap$Separator$drop"]
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
                ArgMap,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::A,
                    ::cxx::r,
                    ::cxx::g,
                    ::cxx::M,
                    ::cxx::a,
                    ::cxx::p,
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
                <std_initializer_list_std_string_AutocxxConcrete as __AmbiguousIfImpl<_>>::infer
            };
            const _: fn() = ::cxx::private::verify_extern_type::<
                ArgMap_Separator,
                (
                    ::cxx::h,
                    ::cxx::e,
                    ::cxx::l,
                    ::cxx::i,
                    ::cxx::b,
                    (),
                    ::cxx::A,
                    ::cxx::r,
                    ::cxx::g,
                    ::cxx::M,
                    ::cxx::a,
                    ::cxx::p,
                    (),
                    ::cxx::S,
                    ::cxx::e,
                    ::cxx::p,
                    ::cxx::a,
                    ::cxx::r,
                    ::cxx::a,
                    ::cxx::t,
                    ::cxx::o,
                    ::cxx::r,
                ),
            >;
            const _: fn() =
                ::cxx::private::verify_extern_kind::<ArgMap_Separator, ::cxx::kind::Trivial>;
        };
    }
    #[allow(unused_imports)]
    use bindgen::root;
    pub use cxxbridge::autocxx_make_string_0xfdd77103f48d7970 as make_string;
    pub use bindgen::root::std_initializer_list_std_string_AutocxxConcrete;
    pub mod helib {
        pub use super::bindgen::root::helib::ArgMap;
        pub use super::bindgen::root::helib::ArgMap_Separator;
    }
}
