pub fn version() -> String {
    zno_seal_sys::bgv::ffi::version()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = version();
        assert_eq!(version, "4.1.1");
    }
}