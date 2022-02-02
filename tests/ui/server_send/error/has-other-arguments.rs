use hyper_tracing_attributes::server_send;

#[allow(unused_braces)]
#[server_send(input % 2)]
fn f(a: u32) -> u32 {a}

fn main() {}
