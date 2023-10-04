// #[cfg(feature = "bindgen")]
// use bindgen::{MacroTypeVariation, RustTarget};
use std::io::Write;
use std::path::PathBuf;
#[cfg(not(feature = "autocxx"))]
use std::process;
use std::{env, fs};

// const INCLUDES: &str = "
// #include <openssl/aes.h>
// #include <openssl/asn1.h>
// #include <openssl/bio.h>
// #include <openssl/cmac.h>
// #include <openssl/conf.h>
// #include <openssl/crypto.h>
// #include <openssl/dh.h>
// #include <openssl/dsa.h>
// #include <openssl/ec.h>
// #include <openssl/err.h>
// #include <openssl/evp.h>
// #include <openssl/hmac.h>
// #include <openssl/objects.h>
// #include <openssl/opensslv.h>
// #include <openssl/pem.h>
// #include <openssl/pkcs12.h>
// #include <openssl/pkcs7.h>
// #include <openssl/rand.h>
// #include <openssl/rsa.h>
// #include <openssl/safestack.h>
// #include <openssl/sha.h>
// #include <openssl/ssl.h>
// #include <openssl/stack.h>
// #include <openssl/x509.h>
// #include <openssl/x509_vfy.h>
// #include <openssl/x509v3.h>

// // this must be included after ssl.h for libressl!
// #include <openssl/srtp.h>

// #if !defined(LIBRESSL_VERSION_NUMBER) && !defined(OPENSSL_IS_BORINGSSL)
// #include <openssl/cms.h>
// #endif

// #if !defined(OPENSSL_IS_BORINGSSL)
// #include <openssl/comp.h>
// #include <openssl/ocsp.h>
// #endif

// #if !defined(LIBRESSL_VERSION_NUMBER) && OPENSSL_VERSION_NUMBER >= 0x10100000
// #include <openssl/kdf.h>
// #endif

// #if OPENSSL_VERSION_NUMBER >= 0x30000000
// #include <openssl/provider.h>
// #endif

// #if defined(LIBRESSL_VERSION_NUMBER) || defined(OPENSSL_IS_BORINGSSL)
// #include <openssl/poly1305.h>
// #endif
// ";

#[cfg(feature = "autocxx")]
pub fn run(include_dirs: &[PathBuf]) {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    let path = std::path::PathBuf::from("./../src"); // include path

    // This assumes all C++ bindings are in src/main.rs
    let mut builder = autocxx_build::Builder::new("./../src/main.rs", &[&path]).build().unwrap();
    builder.flag_if_supported("-std=c++17")
        .compile("helib-autocxx"); // arbitrary library name, pick anything
        // .parse_callbacks(Box::new(OpensslCallbacks))
        // .rust_target(RustTarget::Stable_1_47)
        // .ctypes_prefix("::libc")
        // .raw_line("use libc::*;")
        // .raw_line("type evp_pkey_st = EVP_PKEY;")
        // .allowlist_file(".*/openssl/[^/]+\\.h")
        // .allowlist_recursively(false)
        // // libc is missing pthread_once_t on macOS
        // .blocklist_type("CRYPTO_ONCE")
        // .blocklist_function("CRYPTO_THREAD_run_once")
        // // we don't want to mess with va_list
        // .blocklist_function("BIO_vprintf")
        // .blocklist_function("BIO_vsnprintf")
        // .blocklist_function("ERR_vset_error")
        // .blocklist_function("ERR_add_error_vdata")
        // .blocklist_function("EVP_KDF_vctrl")
        // .blocklist_type("OSSL_FUNC_core_vset_error_fn")
        // .blocklist_type("OSSL_FUNC_BIO_vprintf_fn")
        // .blocklist_type("OSSL_FUNC_BIO_vsnprintf_fn")
        // // Maintain compatibility for existing enum definitions
        // .rustified_enum("point_conversion_form_t")
        // // Maintain compatibility for pre-union definitions
        // .blocklist_type("GENERAL_NAME")
        // .blocklist_type("GENERAL_NAME_st")
        // .blocklist_type("EVP_PKEY")
        // .blocklist_type("evp_pkey_st")
        // .layout_tests(false)
        // .header_contents("includes.h", INCLUDES);

    // for include_dir in include_dirs {
    //     builder = builder
    //         .clang_arg("-I")
    //         .clang_arg(include_dir.display().to_string());
    // }

    // builder
    //     .generate()
    //     .unwrap()
    //     .write_to_file(out_dir.join("bindgen.rs"))
    //     .unwrap();
}
