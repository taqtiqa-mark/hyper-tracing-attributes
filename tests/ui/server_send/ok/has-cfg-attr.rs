#![allow(unused_imports)]

use tracing_attributes_http::server_send;
use tracing::Level;

#[allow(unused_variables)]
#[cfg_attr(feature = "tracing", server_send(level = Level::INFO))]
#[cfg_attr(feature = "tracing", trace_field(var = 0))]
fn f(i: u32, n: u32, put: u32) -> u32 {
        let var = i;
        put
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

    f(4,3,2);
}
