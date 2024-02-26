#[test]
fn test_snapshot() {
    let value = "hello world";
    insta::assert_snapshot!(value);
}
