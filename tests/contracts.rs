#[test]
fn contracts() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/contracts/error/*.rs");
    t.pass("tests/ui/contracts/ok/*.rs");
}
