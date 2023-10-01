extern crate libc;

use libc::c_ulong;

extern "C" {
    pub fn helib_version_num() -> c_ulong;
}

#[test]
fn version_works() {
    unsafe {
        println!("{:#x}", helib_version_num());
        assert!(helib_version_num() > 0);
    }
}