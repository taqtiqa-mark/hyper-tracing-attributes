use hyper_tracing_attributes::server_send;

#[server_send(level=trace, name="B", skip=[self], target="bulls-eye")]
fn f(self) {self}

fn main() { }
