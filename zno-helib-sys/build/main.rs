fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src/helib_pack/include"); // include path
    let mut b = autocxx_build::Builder::new("src/main.rs", &[&path])
        .extra_clang_args(&["-std=c++17"])
        .build()?;
        // This assumes all your C++ bindings are in main.rs
    b.flag_if_supported("-std=c++17")
     .compile("autocxx-demo"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/helib_pack/include/helib/helib.h");
    // Add instructions to link to any C++ libraries you need.
    Ok(())
}
