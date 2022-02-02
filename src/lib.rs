use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;

//#[cfg(feature = "debug")]
// #[macro_use]
// use tracing::{event, Level};

mod contracts;
mod server_send;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn contracts(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = contracts::parse(args.into(), item.into());
    eprintln!("Ast: {:#?}",ast);
    let model = contracts::analyze(ast);
    eprintln!("Model: {:#?}",model);
    let ir = contracts::lower(model);
    eprintln!("Ir: {:#?}",ir);
    let rust = contracts::codegen(ir);
    eprintln!("Rust: {:#?}",rust);
    rust.into()
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn server_send(metadata: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the list of arguments.
    let meta = metadata.clone();
    let mut meta_args = syn::parse_macro_input!(meta as crate::server_send::parse::Args);

    let ast = server_send::parse(metadata.into(), item.into());
    eprintln!("Ast: {:#?}",ast);
    let model = server_send::analyze(ast);
    eprintln!("Model: {:#?}",model);
    let ir = server_send::lower(model);
    eprintln!("Ir: {:#?}",ir);
    let rust = server_send::codegen(ir);
    eprintln!("Rust: {:#?}",rust);
    rust.into()
}
