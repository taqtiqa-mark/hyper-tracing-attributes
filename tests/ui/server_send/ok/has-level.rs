#![allow(unused_imports)]

use tracing_attributes_http::server_send;
use tracing::Level;

#[allow(unused_variables)]
#[server_send(level = Level::INFO)]
#[trace_field(var = 0)]
fn f(i: u32, n: u32, put: u32) -> u32 {
        let span = tracing::Span::current();
        tracing::info!("Here");
        span.record(
                "http.status_code",
                &format!("{:?}", i).as_str()
        );
        let var = i;
        put
    }

#[tokio::main]
async fn main() {
    use tracing_subscriber::fmt;
    use tracing_subscriber::{filter::LevelFilter, prelude::*};

    // Configure a custom event formatter
    let format = fmt::format()
        .with_level(false) // don't include levels in formatted output
        .with_target(true) // don't include targets
        .with_thread_ids(true) // include the thread ID of the current thread
        .with_thread_names(true) // include the name of the current thread
        .compact(); // use the `Compact` formatting style.

    // A general-purpose tracing layer.
    let fmt_layer = tracing_subscriber::fmt::layer()
        .event_format(format);


    // Build a subscriber combining stdout and stderr layers.
    tracing_subscriber::registry()
        .with(fmt_layer)
        .init();

    // Create a `fmt` subscriber that uses our custom event format, and set it
    // as the default.
    // By binding the result to an unused variable, the lifetime of the variable
    // matches the containing block, reporting traces and metrics during the whole
    // execution.

    // let _tracer = tracing_subscriber::fmt()
    //     .with_writer(std::io::stderr)
    //     .event_format(format)
    //     .init();
    // let collector = tracing_subscriber::fmt()
    //     .with_writer(std::io::stderr)
    //     .event_format(format)
    //     .finish();

    // tracing::collect::with_default(collector, || {
    //     tracing::info!("This event will be printed to `stderr`.");
    // });

    tracing::info!("Before");
    f(4,3,2);
    tracing::info!("After");

}
