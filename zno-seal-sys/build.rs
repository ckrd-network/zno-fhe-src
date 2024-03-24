use std::collections::HashSet;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> miette::Result<()> {

    // Check if the "static" feature is enabled
    if env::var_os("CARGO_FEATURE_STATIC").is_some() {
        // If "static" feature is enabled, set BUILD_SHARED_LIBS to control the build process accordingly.
        std::env::set_var("BUILD_SHARED_LIBS", "OFF");
        println!("cargo:rustc-env=BUILD_SHARED_LIBS=OFF");
    }

    // cxx_build::CFG.exported_header_prefixes = vec!["seal"];
    // Determine the project directory
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let ffi_dir = Path::new(&project_dir).join("ffi");
    let seal_dir = Path::new(&project_dir).join("src").join("seal_pack");

    let seal_lib_dir = seal_dir.join("lib");
    let seal_include_dir = seal_dir.join("include");

    let cxx_include_path = format!("{}/target/cxxbridge", project_dir);

    // Compile ffi_wrapper.cpp separately
    let cpp_source = ffi_dir.join("ffi_wrapper.cpp");
    let cpp_output = ffi_dir.join("ffi_wrapper"); // Output binary name

    // Retrieve the CARGO_MANIFEST_DIR and OUT_DIR environment variables
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();

    let source_dir = Path::new(&manifest_dir).join("src/seal_pack/include/SEAL-4.1/seal");
    let target_dir = Path::new(&manifest_dir).join("ffi/seal");

    // Recursively copy files from source to target directory
    if let Err(e) = copy_dir_to(&source_dir, &target_dir) {
        panic!("Failed to copy header files from {} to {}: {}", source_dir.display(), target_dir.display(), e);
    }
    let source_dir = Path::new(&manifest_dir).join("src/seal_pack/include/SEAL-4.1/gsl");
    let target_dir = Path::new(&manifest_dir).join("ffi/gsl");

    // Recursively copy files from source to target directory
    if let Err(e) = copy_dir_to(&source_dir, &target_dir) {
        panic!("Failed to copy header files from {} to {}: {}", source_dir.display(), target_dir.display(), e);
    }

    // Output the linker flags for the compiled wrapper C++ source
    println!("cargo:rustc-link-search=native={}", ffi_dir.display());
    // println!("cargo:rustc-link-lib=dylib=ffi_wrapper");

    // Consider moving these triggers to an rebuild script.
    // println!("cargo:rerun-if-changed={}",ffi_dir.join("ffi_wrapper.h").display());
    // println!("cargo:rerun-if-changed={}",ffi_dir.join("ffi_wrapper.cpp").display());

    // Compile cxx generated bindings.  This is the name of the Rust FFI library
    // that includes the generated Rust bindings for C++ code.
    // Used to link Rust code with the upstream C++ code.
    let path: PathBuf = cpp_source.to_string_lossy().into_owned().into();
    let mut cc_build = cxx_build::bridge("src/seal/bgv.rs");
    // Include the directory where cxx generates the cxxbridge sources.
    // This directory will contain the rust/cxx.h header.
    println!("cargo:include={}", ffi_dir.display());

    cc_build.file(path)
            .include(cxx_include_path)
            .include(ffi_dir)
            .flag_if_supported("-std=c++17")
            .compile("seal-context"); // compile cxx generated bindings

    // Consider moving these triggers to an rebuild script.
    // println!("cargo:rerun-if-changed=src/lib.rs");
    // println!("cargo:rerun-if-changed=ffi/seal/seal.h");

    // Add instructions to link to any C++ libraries you need.

    let rtfcts = Build::new().artifacts();

    // Recursively copy files from ./src/seal_pack/lib to ./libs directory
    if let Err(e) = copy_dir_to(&rtfcts.lib_dir, &rtfcts.libs_dir) {
        panic!("Failed to copy library files to ./libs: {}", e);
    }

    // Recursively copy files from ./src/seal_pack/lib to integration tests directory
    if let Err(e) = copy_dir_to(&rtfcts.lib_dir, &rtfcts.tests_dir) {
        panic!("Failed to copy library files to integration tests: {}", e);
    }

    // Recursively copy files from ./src/seal_pack/lib to integration tests directory
    if let Err(e) = copy_dir_to(&rtfcts.lib_dir, &rtfcts.tests_target_dir) {
        panic!("Failed to copy library files to target integration tests: {}", e);
    }

    rtfcts.print_cargo_metadata();

    Ok(())
}

pub fn source_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("seal")
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Function to recursively copy directories
fn copy_dir_to(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.is_dir() {
        fs::create_dir_all(dst)?;
    }

    for entry_result in src.read_dir()? {
        let entry = entry_result?;
        let file_type = entry.file_type()?;
        let src_path = src.join(entry.file_name());
        let dst_path = dst.join(entry.file_name());

        if file_type.is_dir() {
            copy_dir_to(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
pub struct Build {
    out_dir: Option<PathBuf>,
    target: Option<String>,
    host: Option<String>,
}

pub struct Artifacts {
    out_dir: PathBuf,
    package_dir: PathBuf,
    include_dir: PathBuf,
    lib_dir: PathBuf,
    libs_dir: PathBuf,
    tests_dir: PathBuf,
    tests_target_dir: PathBuf,
    bin_dir: PathBuf,
    share_dir: PathBuf,
    libs: Vec<String>,
    target: String,
}

impl Build {
    pub fn new() -> Build {
        Build {
            out_dir: env::var_os("OUT_DIR").map(|s| PathBuf::from(s)),
            target: env::var("TARGET").ok(),
            host: env::var("HOST").ok(),
        }
    }

    pub fn out_dir<P: AsRef<Path>>(&mut self, path: P) -> &mut Build {
        self.out_dir = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn target(&mut self, target: &str) -> &mut Build {
        self.target = Some(target.to_string());
        self
    }

    pub fn host(&mut self, host: &str) -> &mut Build {
        self.host = Some(host.to_string());
        self
    }

    fn cmd_make(&self) -> Command {
        let host = &self.host.as_ref().expect("HOST dir not set")[..];
        if host.contains("dragonfly")
            || host.contains("freebsd")
            || host.contains("openbsd")
            || host.contains("solaris")
            || host.contains("illumos")
        {
            Command::new("gmake")
        } else {
            Command::new("make")
        }
    }

    #[cfg(windows)]
    fn check_env_var(&self, var_name: &str) -> Option<bool> {
        env::var_os(var_name).map(|s| {
            if s == "1" {
                // a message to stdout, let user know asm is force enabled
                println!(
                    "{}: nasm.exe is force enabled by the \
                    'ZNO_RUST_USE_NASM' env var.",
                    env!("CARGO_PKG_NAME")
                );
                true
            } else if s == "0" {
                // a message to stdout, let user know asm is force disabled
                println!(
                    "{}: nasm.exe is force disabled by the \
                    'ZNO_RUST_USE_NASM' env var.",
                    env!("CARGO_PKG_NAME")
                );
                false
            } else {
                panic!(
                    "The environment variable {} is set to an unacceptable value: {:?}",
                    var_name, s
                );
            }
        })
    }

    pub fn artifacts(&mut self) -> Artifacts {
        let target = &self.target.as_ref().expect("TARGET dir not set")[..];
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let workspace_dir = Path::new(&manifest_dir).parent().expect("Cannot find workspace directory").to_path_buf();

        let out_dir = self.out_dir.as_ref().expect("OUT_DIR not set").to_path_buf();
        // Generate files under the -sys crate src folder
        let install_dir = env::var_os("CARGO_MANIFEST_DIR")
            .map(|s| PathBuf::from(s))
            .unwrap()
            .join("src");

        // Define your libs folder relative to the crate root
        let libs_dir = env::var_os("CARGO_MANIFEST_DIR")
            .map(|s| PathBuf::from(s))
            .unwrap()
            .join("libs");

        // Make sure `libs` directory exists
        fs::create_dir_all(&libs_dir).expect("Failed to create 'libs/' directory");

        // Construct the path to the 'libs' directory adjacent to the test binaries
        let tests_dir = workspace_dir.join("target").join("debug").join("libs");
        let tests_target_dir = workspace_dir.join("target").join(target).join("debug").join("libs");

        // Create the "libs" directory in the target location if it doesn't exist
        fs::create_dir_all(&tests_dir).expect("Could not create target 'libs' directory");
        fs::create_dir_all(&tests_target_dir).expect("Could not create 'libs' directory");

        let libs = if target.contains("msvc") {
            vec!["sealw".to_string(), "gsl".to_string()]
        } else {
            vec!["seal".to_string(), "gsl".to_string()]
        };

        let pd = install_dir.join("seal_pack");
        Artifacts {
            out_dir: out_dir,
            package_dir: pd.clone(),
            lib_dir: pd.clone().join("lib"),
            libs_dir,
            tests_dir,
            tests_target_dir,
            bin_dir: pd.clone().join("bin"),
            share_dir: pd.clone().join("share"),
            include_dir:pd.clone().join("include"),
            libs,
            target: target.to_string(),
        }
    }
}

impl Artifacts {
    pub fn include_dir(&self) -> &Path {
        &self.include_dir
    }

    // The libraries packaged from the src crate.
    // These are not distributed with the crate.
    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    // The src/libs directory that is distributed with the crate.
    // Contains the 3rd party libraries that are linked to.
    // The library code points here at runtime to link without
    // requiring changes to the user system.
    pub fn libs_dir(&self) -> &Path {
        &self.libs_dir
    }

    pub fn tests_dir(&self) -> &Path {
        &self.tests_dir
    }

    pub fn tests_target_dir(&self) -> &Path {
        &self.tests_target_dir
    }

    pub fn bin_dir(&self) -> &Path {
        &self.bin_dir
    }

    pub fn package_dir(&self) -> &Path {
        &self.package_dir
    }

    pub fn out_dir(&self) -> &Path {
        &self.out_dir
    }

    pub fn share_dir(&self) -> &Path {
        &self.share_dir
    }

    pub fn libs(&self) -> &[String] {
        &self.libs
    }

    pub fn target(&self) -> String {
        self.target.clone()
    }

    pub fn print_cargo_metadata(&self) {
        // If you need to link to the libraries, inform cargo to link them
        println!("cargo:rustc-link-search=native={}", self.libs_dir.display());

        // Set rpath for Library B
        #[cfg(target_os = "macos")]
        println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path/../libs:@loader_path/libs:@loader_path");

        #[cfg(target_os = "linux")]
        println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../libs:$ORIGIN/libs:$ORIGIN");

        // On Windows, rpath concept doesn't exist.
        // Instead the end user is responsible for setting up their system so these libraries are available to the linker.

        if env::var_os("CARGO_FEATURE_STATIC").is_some() {
            // If "static" feature is enabled, set rustc-link-lib=... to control the build process accordingly.
            println!("cargo:rustc-link-lib=static=seal");
        } else {
            println!("cargo:rustc-link-lib=dylib=seal");
        }
        println!("cargo:rustc-link-lib=dylib=gmp");
        println!("cargo:rustc-link-lib=dylib=ntl");

        // Link the C++ standard library statically, if desired
        println!("cargo:rustc-link-search=native=/usr/lib/gcc/x86_64-linux-gnu/13");
        println!("cargo:rustc-link-lib=static=stdc++");

        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.libs_dir.display());
        if self.target.contains("msvc") {
            println!("cargo:rustc-link-lib=user32");
        } else if self.target == "wasm32-wasi" {
            println!("cargo:rustc-link-lib=wasi-emulated-signal");
            println!("cargo:rustc-link-lib=wasi-emulated-process-clocks");
            println!("cargo:rustc-link-lib=wasi-emulated-mman");
            println!("cargo:rustc-link-lib=wasi-emulated-getpid");
        }
    }

    pub fn env_inner(name: &str) -> Option<OsString> {
        let var = env::var_os(name);
        println!("cargo:rerun-if-env-changed={}", name);

        match var {
            Some(ref v) => println!("{} = {}", name, v.to_string_lossy()),
            None => println!("{} unset", name),
        }

        var
    }

    pub fn get_env(name: &str) -> Option<OsString> {
        let prefix = env::var("TARGET").unwrap().to_uppercase().replace('-', "_");
        let prefixed = format!("{}_{}", prefix, name);
        Self::env_inner(&prefixed).or_else(|| Self::env_inner(name))
    }

}
