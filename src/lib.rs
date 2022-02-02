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
    let model = contracts::analyze(ast);
    let ir = contracts::lower(model);
    let rust = contracts::codegen(ir);
    eprintln!("{}",rust);
    rust.into()
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn server_send(args: TokenStream, item: TokenStream) -> TokenStream {
    let ast = server_send::parse(args.into(), item.into());
    let model = server_send::analyze(ast);
    let ir = server_send::lower(model);
    let rust = server_send::codegen(ir);
    eprintln!("{}",rust);
    rust.into()
}
