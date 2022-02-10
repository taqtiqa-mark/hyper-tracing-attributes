use http_tracing_attributes::server_send;

#[server_send(trace, "Server")]
#[trace_field(b)]
fn f() {}

fn main() {}
