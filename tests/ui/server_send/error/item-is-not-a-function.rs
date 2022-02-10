use http_tracing_attributes::server_send;

#[server_send(trace, "Some")]
struct S;

fn main() {}
