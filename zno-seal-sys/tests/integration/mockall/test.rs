use mockall::mock;

mock! {
    MyTrait {
        fn my_function(&self, input: u32) -> u32;
    }
}

#[test]
fn test_mock() {
    let mut mock = MockMyTrait::new();
    mock.expect_my_function().times(1).returning(|x| x + 1);

    assert_eq!(mock.my_function(1), 2);
}
