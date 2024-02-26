
fn main() {
    let version = zno_helib_sys::helib::version::version();
    println!("HElib version: {}", version);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_version() {
        let version = zno_helib_sys::helib::version::version();
        assert_eq!(version, "2.2.0");
    }
}
