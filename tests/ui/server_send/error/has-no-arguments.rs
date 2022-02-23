use tracing_attributes_http::server_send;

#[allow(unused_braces)]
#[server_send]
fn f(a: u32) -> u32 {
    a
}

fn main() {}
