use proptest::prelude::*;

proptest! {
    #[test]
    fn test_property(a: u32, b: u32) {
        prop_assert_eq!(a + b, b + a);
    }
}
