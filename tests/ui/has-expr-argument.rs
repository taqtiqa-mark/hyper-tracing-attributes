use tracing_attributes_http::server_send;

#[server_send(true)]
fn f() {}

fn main() {}
