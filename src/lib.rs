//! Procedural macro attributes for instrumenting functions with [`tracing`], in
//! HTTP use cases.
//!

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod server_send;
mod utility;

/// Add tracing instrumentation attribute: Server-Send
///
/// # Example
///
/// ```ignore
/// use tracing_attributes_http::*;
///
/// #[server_send(level = tracing::Level::TRACE, name = "Server::encode", skip = [dst, msg])]
/// fn traced(mut msg: String, dst: &mut Vec<u8>) {
///     // ...
/// }
/// ```
#[proc_macro_attribute]
#[proc_macro_error]
pub fn server_send(metadata: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the list of arguments.
    let meta = metadata.clone();
    let meta_args = syn::parse_macro_input!(meta as crate::server_send::parse::Args);

    let ast = server_send::parse(metadata.into(), item.into());
    let model = server_send::analyze(ast, meta_args);
    let ir = server_send::lower(model);
    let rust = server_send::codegen(ir);
    rust.into()
}
