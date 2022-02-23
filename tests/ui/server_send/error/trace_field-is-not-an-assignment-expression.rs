use tracing_attributes_http::server_send;

#[server_send(trace, "Server")]
#[trace_field(b)]
fn f() {}

fn main() {}
