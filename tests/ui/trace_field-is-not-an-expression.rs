use tracing_attributes_http::server_send;

#[server_send]
#[trace_field(struct)]
fn f() {}

fn main() {}
