pub fn version() -> String {
    crate::seal::bgv::ffi::version()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = version();
        assert_eq!(version, "2.2.0");
    }
}