#![allow(unused_braces)]
use http_tracing_attributes::server_send;

#[server_send(trace, "B", skip(self))]
fn f(a: u32) -> u32 {
    a
}

fn main() {}
