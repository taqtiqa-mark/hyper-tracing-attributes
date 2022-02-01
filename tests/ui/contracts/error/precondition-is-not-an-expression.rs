use hyper_tracing_attributes::contracts;

#[contracts]
#[precondition(struct)]
fn f() {}

fn main() {}
