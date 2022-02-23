use tracing_attributes_http::server_send;

#[server_send(trace, "Some")]
struct S;

fn main() {}
