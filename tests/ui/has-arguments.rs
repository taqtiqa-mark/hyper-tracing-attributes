use tracing_attributes_http::server_send;

#[server_send(a, b)]
fn f() {}

fn main() {}
