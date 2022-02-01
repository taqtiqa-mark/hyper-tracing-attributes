#![allow(unused_braces)]
use hyper_tracing_attributes::server_send;

#[server_send(trace, "B", skip(self))]
fn f(self) {self}


fn main() {}
