use std::collections::HashSet;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src/helib_pack/include"); // include path
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path])
        .extra_clang_args(&["-std=c++17"])
        .build()?;
        // This assumes all your C++ bindings are in main.rs
    b.flag_if_supported("-std=c++17")
     .compile("helib-autocxx"); // arbitrary library name, pick anything

    // // Compile cxx generated bindings
    // let mut cc_build = cxx_build::bridge("src/cxx-bridges/issue-2.rs");
    // cc_build.include("src/helib_pack/include")
    //         .flag_if_supported("-std=c++17")
    //         .compile("helib-issue-2"); // compile cxx generated bindings

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/helib_pack/include/helib/helib.h");

    // Add instructions to link to any C++ libraries you need.

    let rtfcts = Build::new().artifacts();
    rtfcts.print_cargo_metadata();

    Ok(())
}

pub fn source_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("helib")
}

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub struct Build {
    out_dir: Option<PathBuf>,
    target: Option<String>,
    host: Option<String>,
}

pub struct Artifacts {
    package_dir: PathBuf,
    include_dir: PathBuf,
    lib_dir: PathBuf,
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

    // #[cfg(windows)]
    // fn is_nasm_ready(&self) -> bool {
    //     self.check_env_var("ZNO_RUST_USE_NASM")
    //         .unwrap_or_else(|| {
    //             // On Windows, use cmd `where` command to check if nasm is installed
    //             let wherenasm = Command::new("cmd")
    //                 .args(&["/C", "where nasm"])
    //                 .output()
    //                 .expect("Failed to execute `cmd`.");
    //             wherenasm.status.success()
    //         })
    // }

    // #[cfg(not(windows))]
    // fn is_nasm_ready(&self) -> bool {
    //     // We assume that nobody would run nasm.exe on a non-windows system.
    //     false
    // }

    pub fn artifacts(&mut self) -> Artifacts {
        let target = &self.target.as_ref().expect("TARGET dir not set")[..];
        let out_dir = self.out_dir.as_ref().expect("OUT_DIR not set");
        // Generate files under the -sys crate src folder
        let install_dir = env::var_os("CARGO_MANIFEST_DIR")
            .map(|s| PathBuf::from(s))
            .unwrap()
            .join("src");

        let libs = if target.contains("msvc") {
            vec!["helibw".to_string(), "gmp".to_string(), "ntl".to_string()]
        } else {
            vec!["helib".to_string(), "gmp".to_string(), "ntl".to_string()]
        };

        let pd = install_dir.join("helib_pack");
        Artifacts {
            package_dir: pd.clone(),
            lib_dir: pd.clone().join("lib"),
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

    pub fn lib_dir(&self) -> &Path {
        &self.lib_dir
    }

    pub fn bin_dir(&self) -> &Path {
        &self.bin_dir
    }

    pub fn package_dir(&self) -> &Path {
        &self.package_dir
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
        println!("cargo:rustc-link-search=native={}", self.lib_dir.display());

        let libdirs = vec![self.lib_dir().to_path_buf()];
        let kind = Self::determine_mode(&libdirs, &self.libs);
        for lib in self.libs.clone().into_iter() {
            println!("cargo:rustc-link-lib={}={}", kind, lib);
        }

        println!("cargo:include={}", self.include_dir.display());
        println!("cargo:lib={}", self.lib_dir.display());
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

    /// Given a libdir (where artifacts are located) as well as the name
    /// of the libraries we're linking to, figure out whether we should link them
    /// statically or dynamically.
    fn determine_mode(libdirs: &Vec<PathBuf>, libs: &[String]) -> &'static str {
        // First see if a mode was explicitly requested
        let kind = Self::get_env("HELIB_STATIC");
        match kind.as_ref().and_then(|s| s.to_str()) {
            Some("0") => return "dylib",
            Some(_) => return "static",
            None => {}
        }

        // Next, see what files we actually have to link against, and see what our
        // possibilities even are.
        let mut files = HashSet::new();
        for dir in libdirs {
            for path in dir
                .read_dir()
                .unwrap()
                .map(|e| e.unwrap())
                .map(|e| e.file_name())
                .filter_map(|e| e.into_string().ok())
            {
                files.insert(path);
            }
        }
        let can_static = libs
            .iter()
            .all(|l| files.contains(&format!("lib{}.a", l)) || files.contains(&format!("{}.lib", l)));
        let can_dylib = libs.iter().all(|l| {
            files.contains(&format!("lib{}.so", l))
                || files.contains(&format!("{}.dll", l))
                || files.contains(&format!("lib{}.dylib", l))
        });
        match (can_static, can_dylib) {
            (true, false) => return "static",
            (false, true) => return "dylib",
            (false, false) => {
                panic!(
                    "libdir does not contain the required files \
                    to either statically or dynamically link HElib"
                );
            }
            (true, true) => {}
        }

        // Ok, we've got not explicit preference and can *either* link statically or
        // link dynamically. In the interest of "security upgrades" and/or "best
        // practices with security libs", let's link dynamically.
        "dylib"
    }

}
