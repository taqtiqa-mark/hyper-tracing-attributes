use tracing_attributes_http::server_send;

#[allow(unused_braces)]
#[server_send(level=trace, name="B", skip="self", target="bulls-eye")]
fn f(a: u32) -> u32 {
    a
}

fn main() {}
