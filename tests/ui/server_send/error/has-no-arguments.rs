use hyper_tracing_attributes::server_send;

#[server_send]
fn f(self) {self}

fn main() {}
