pub enum Schema {
    None,
    Bfv,
    Ckks,
    Bgv,
}

impl Default for Schema {
    fn default() -> Self {
        Schema::Bgv
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let schema = Schema::default();
        assert_eq!(schema, Schema::Bgv);
    }
}

// Compare this snippet from zno-seal-sys/src/seal/bgv.rs:
// pub struct Bgv;
// impl Default for Bgv {
//     fn default() -> Self {
//         Bgv
//     }
// }