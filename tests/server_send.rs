#[test]
fn server_send() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/server_send/error/*.rs");
    t.pass("tests/ui/server_send/ok/*.rs");
}
