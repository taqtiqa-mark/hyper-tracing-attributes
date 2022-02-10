#![allow(dead_code)]

use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use syn::{Expr, Item, ItemFn};
use darling;

pub type Ast = ItemFn;

/// Describes the data that directly corresponds to the attributes of the
/// `server_send`-proc-macro.
/// This is the parsing target for the
/// crate "darling" which works together with `syn`.
/// FromMeta implements "from_list"-factory-method (`darling` crate).
#[derive(Debug, darling::FromMeta)]
struct TaskAttributes {
    #[darling(default)]
    level: Option<String>,
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    skip: Option<String>,
    #[darling(default)]
    fields: Option<String>,
}

// #[derive(Debug, darling::FromMeta)]
// struct InstrumentAttributes {
//     #[darling(default)]
//     level: Option<syn::Expr>,
//     #[darling(default)]
//     name: Option<syn::Expr>,
//     #[darling(default)]
//     skip: Option<syn::Expr>,
//     #[darling(default)]
//     fields: Option<syn::Expr>,
// }

impl TaskAttributes {
    /// Maps property `skip` of `TaskAttributes` from a
    /// comma-separated string to a vector of strings.
    fn skip_params(&self) -> Vec<String> {
        self.skip
            .as_ref()
            .map(|s| s.clone())
            .map(|s| {
                s.split(",")
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
            })
            .unwrap_or(vec![])
            .into_iter()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }

    /// Maps property `read` of `TaskAttributes` from a
    /// comma-separated string to a vector of strings.
    fn fields_params(&self) -> Vec<String> {
        self.fields
            .as_ref()
            .map(|s| s.clone())
            .map(|s| {
                s.split(",")
                    .into_iter()
                    .map(|s| s.to_owned())
                    .collect::<Vec<String>>()
            })
            .unwrap_or(vec![])
            .into_iter()
            .map(|s| s.trim().to_owned())
            .collect::<Vec<String>>()
    }
}

// /// Invoked like this:
// /// - `#[task(write = "data1, data2")]`
// /// - `#[task(write = "data1", read = "data2")]`
// /// - `#[task(read = "data2")]`
// /// If a parameter is write it is automatically also read.
// #[proc_macro_attribute]
// pub fn task(args: TokenStream, item: TokenStream) -> TokenStream {
//     // we do nothing here; just a marker for the super macro
//     let attr_args = parse_macro_input!(args as AttributeArgs);
//     let attrs: TaskAttributes =
//         TaskAttributes::from_list(&attr_args).expect("#[task] could not be parsed!");
//     println!("read_params: {:#?}", attrs.read_params());
//     println!("write_params: {:#?}", attrs.write_params());
//     item
// }

// Attribute arguments that are assignment expressions.
#[derive(Debug)]
pub struct Args {
    vars: Vec<syn::Expr>
}

// Parse the attribute data once we know it contains ExprAssign
impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        // parses a=1,b="d",c=f(), where a,b and c are ExprAssign
        let vars = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

// Parse attribute arguments then count.
// This utility/no-op function is required by the fact
// `syn::parse_macro_input!` must be called in a function that returns
// proc_macro::TokenStream.
pub fn parse_args(metadata: proc_macro::TokenStream) -> proc_macro::TokenStream {
    const ERROR: &str = "this attribute takes from one to four arguments";
    const HELP: &str = "use `#[server_send(level=trace, name=\"Name\", skip=\"a, b\")]`";

    // eprintln!("{:?}", metadata);

    if metadata.is_empty() {
        // ../tests/ui/server_send/error/has-no-arguments.rs
        abort_call_site!(ERROR; help = HELP)
    }

    let md = metadata.clone();
    let args = syn::parse_macro_input!(md as Args);

    // eprintln!("{}",args.vars.len());

    // eprintln!("{:?}", args);

    for expr in &args.vars {
        match expr {
            Expr::Array(expr) => {
                // ../tests/ui/server_send/error/has-expression-arguments.rs
                abort!(expr, ERROR; help = HELP)
            }
            Expr::Assign(_expr) => { }
            _ => {
                // ../tests/ui/server_send/error/has-other-arguments.rs
                abort!(expr, ERROR; help = HELP)
            }
        }
    }
    proc_macro::TokenStream::from(quote::quote! {fn dummy(){}})
}

pub fn parse(metadata: TokenStream, item: TokenStream) -> Ast {

    parse_args(metadata.clone().into());

    match syn::parse2::<Item>(item) {
        Ok(Item::Fn(item)) => item,
        Ok(item) => {
            // ../tests/ui/server_send/error/item-is-not-a-function.rs
            abort!(
                item,
                "item is not a function";
                help = "`#[server_send(...)]` can only be used on functions"
            )
        }
        Err(_) => unreachable!(), // ?
    }
}

// #[cfg(test)]
// mod tests {
//     use quote::quote;

//     use super::*;

//     #[test]
//     fn valid_syntax() {
//         parse(
//             quote!(),
//             quote!(
//                 #[inline]
//                 #[server_send(level = debug)]
//                 fn even_to_odd(x: u32) -> u32 {
//                     x + 1
//                 }
//             ),
//         );
//     }
// }
