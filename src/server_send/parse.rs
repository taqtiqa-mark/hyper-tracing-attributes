#![allow(dead_code)]

use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use syn::{Expr, Item};

pub type Ast = syn::ItemFn;

// Standard parsing of input arguments that are assignment expressions.
#[derive(Debug)]
pub struct Args {
    pub vars: Vec<syn::Expr>
}

// // Custom parsing of metadata arguments
// pub struct MetaArgs {
//     pub vars: Vec<syn::ExprAssign>,
// }

// The MetaArg struct and parser are an alternative to `syn::parse_meta()?`
// No arguments are mandatory, hence `Option<...>`.
// Arguments are optional because we impl Default for MetaArg.
// #[derive(Debug)]
// struct MetaArg {
//     level: Option<syn::PatPath>,
//     name: Option<syn::LitStr>,
//     skip: Option<syn::ExprArray>,
// }

// WIP: requires impl syn::parse::Parse for FieldArg
// Should allow `trace_field(var)` which defaults to `var = Empty`
// #[derive(Debug)]
// struct FieldArg {
//     value: Option<FieldVariant>,
// }

// #[derive(Debug)]
// enum FieldVariant {
//     Path(syn::PatPath),
//     String(syn::LitStr),
// }

// impl Default for FieldArg {
//     fn default () -> Self {
//         use std::str::FromStr;

//         let tokens = proc_macro2::TokenStream::from_str("tracing::field::Empty").expect("Token stream");
//         let path = syn::parse2(tokens).expect("A struct path");
//         Self {value: Some(FieldVariant::Path(path))}
//         //Self
//     }
// }

// impl Default for FieldVariant {
//     fn default() -> Self {
//         FieldVariant::Path(tracing::field::Empty)
//     }
// }

// Parse the surrounding attribute data once we know it contains ExprAssign
impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        // parses a=1,b="d",c=f(), into Expr
        let vars = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

// Parse the attribute argument data
// impl syn::parse::Parse for MetaArgs {
//     fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
//         let attrs = input.call(syn::Attribute::parse_outer)?;
//         let vars: Vec<syn::Attribute> = attrs.into_iter().map(|a| {
//             eprintln!("Attribute: {:#?}", a);
//             if a.path.is_ident("server_send") {
//                 eprintln!("MATCHED")
//             }
//             // match a.parse_meta().expect("Parsed meta attribute") {
//             //     _ => {
//             //         let message = "expected path = \"...\"]";
//             //         //Err(syn::Error::new_spanned(a, message))
//             //         eprintln!("Attr: :#?", a);
//             //     }

//             // }
//             a
//         }).collect();
//         // parses a=1,b="d",c=f(), where a,b and c are ExprAssign
//         Ok(MetaArgs {
//             vars: vars.into_iter().collect(),
//         })
//     }
// }

// impl syn::parse::Parse for MetaArg {
//     fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
//         // Parse the argument name
//         let arg_name: syn::Ident = input.parse()?;
//         if arg_name != "name" {
//             // Same error as before when encountering an unsupported attribute
//             return Err(
//                     syn::Error::new_spanned(
//                         arg_name,
//                         "unsupported server_send attribute, expected `name`",
//                     )
//                 );
//         }

//         // Parse (and discard the span of) the `=` token
//         let _: syn::Token![=] = input.parse()?;

//         // Parse (and discard the span of) the `=` token
//         let _: syn::Token![=] = input.parse()?;

//         Ok(Self { name: Some(name) })
//     }
// }

// Parse attribute arguments then assess correctness.
// This utility function generates a no-op/dummy output, because it is
// required by the fact `syn::parse_macro_input!` must be called in a
// function that returns `proc_macro::TokenStream`.
pub fn parse_args(metadata: proc_macro::TokenStream) -> proc_macro::TokenStream {
    const ERROR: &str = "this attribute takes from one to three assign arguments";
    const HELP: &str = "use `#[server_send(level=Level::WARN, name=\"Name\", skip=[a, b])]`";

    if metadata.is_empty() {
        // ../tests/ui/server_send/error/has-no-arguments.rs
        abort_call_site!(ERROR; help = HELP)
    }

    let md = metadata.clone();
    let args = syn::parse_macro_input!(md as Args);

    if args.vars.len() > 3 {
        // ../tests/ui/server_send/error/has-too-many-arguments.rs
        abort_call_site!(ERROR; help = HELP)
    }

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
            // ../tests/ui/item-is-not-a-function.rs
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
