use tracing_attributes_http::server_send;

#[server_send]
#[trace_field(input)]
fn f(input: bool) -> i32 {
    0
}

#[test]
fn pass() {
    f(true);
}
