use hyper_tracing_attributes::contracts;

#[contracts]
#[precondition(input % 2 == 0)]
fn f(input: u32) {}

// fn f(input : u32)
// {
//     assert! (input % 2 == 0, "violation of precondition `input % 2 == 0`") ;
//     { }
// }

#[tokio::main]
async fn main() {
    use tracing_subscriber::fmt;

    // Configure a custom event formatter
    let format = fmt::format()
    .with_level(false) // don't include levels in formatted output
    .with_target(false) // don't include targets
    .with_thread_ids(true) // include the thread ID of the current thread
    .with_thread_names(true) // include the name of the current thread
    .compact(); // use the `Compact` formatting style.

    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    tracing_subscriber::fmt()
        .event_format(format)
        .init();

    f(4)
}
