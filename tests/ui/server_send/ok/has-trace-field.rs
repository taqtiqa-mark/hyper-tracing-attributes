#![allow(unused_imports)]

use hyper_tracing_attributes::server_send;
use tracing::Level;

#[allow(unused_variables)]
#[server_send(level = Level::DEBUG, name = "Some", skip = "input")]
#[trace_field(var = 0)]
fn f(input: u32) -> u32 {
        let var = input;
        input
    }

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

    f(4);
}
