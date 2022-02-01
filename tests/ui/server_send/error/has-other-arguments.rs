use hyper_tracing_attributes::server_send;

#[server_send(input % 2)]
fn f(self) {}

fn main() {}
