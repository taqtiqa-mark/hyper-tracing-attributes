use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

mod server_send;
mod utility;

/// Add tracing instrumentation attribute: Server-Send
///
/// # Example
///
/// ```
/// use tracing_attributes_http::*;
///
/// #[server_send(level = tracing::Level::TRACE, name = "Server::encode", skip = [dst, msg])
/// fn encode(mut msg: Encode<'_, Self::Outgoing>, dst: &mut Vec<u8>) -> crate::Result<Encoder> {
///     ...
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
