
fn main() {
    let version = zno_seal_sys::bgv::version();
    println!("SEAL version: {}", version);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_version() {
        let version = zno_seal_sys::bgv::version();
        assert_eq!(version, "4.1.0");
    }
}
