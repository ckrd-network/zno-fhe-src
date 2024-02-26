extern crate zno_seal_src;

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    std::env::set_var("BUILD_SHARED_LIBS", "OFF");
    let artifacts = zno_seal_src::Build::new().build();
    artifacts.print_cargo_metadata();

    let out_dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("artifacts");
    File::create(out_dir.join("includef"))
        .unwrap()
        .write_all(artifacts.include_dir().to_str().unwrap().as_bytes())
        .unwrap();
    File::create(out_dir.join("libf"))
        .unwrap()
        .write_all(artifacts.lib_dir().to_str().unwrap().as_bytes())
        .unwrap();
    File::create(out_dir.join("targetf"))
        .unwrap()
        .write_all(env::var("TARGET").unwrap().as_bytes())
        .unwrap();
    File::create(out_dir.join("zno-seal-src-version"))
        .unwrap()
        .write_all(zno_seal_src::version().as_bytes())
        .unwrap();
}
