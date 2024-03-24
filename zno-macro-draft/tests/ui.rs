#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/zno/ui/err/*.rs");
    t.pass("tests/zno/ui/ok/*.rs");
}
