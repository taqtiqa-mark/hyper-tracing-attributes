use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use syn::{Expr, Item, ItemFn};
use darling::{FromMeta};

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

    eprintln!("{}", metadata);

    if metadata.is_empty() {
        // ../tests/ui/server_send/error/has-no-arguments.rs
        abort_call_site!(ERROR; help = HELP)
    }

    let md = metadata.clone();
    let args = syn::parse_macro_input!(md as Args);

    eprintln!("{}",args.vars.len());

    // if args.vars.len() > 3 {
    //     // ../tests/ui/server_send/error/has-too-many-arguments.rs
    //     abort_call_site!(ERROR; help = HELP)
    // }

    eprintln!("{:?}", args);

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
    // let expr: syn::Expr = syn::parse2(metadata.clone().into()).expect("ExprAssign parsing");

    // eprintln!("{:?}", expr);

    // let attr_args = syn::parse_macro_input!(metadata as syn::AttributeArgs);

    // eprintln!("{:?}", attr_args);

    // // let attr_args: syn::NestedMeta = args.vars.into();
    // let attrs: TaskAttributes =
    //     TaskAttributes::from_list(&attr_args).expect("#[server_send(...)] could not be parsed!");

    // eprintln!("{:?}", attrs);

    // eprintln!("skip_params: {:#?}", attrs.skip_params());
    // eprintln!("fields_params: {:#?}", attrs.fields_params());

    // if attr_args.len() > 4 {
    //     // ../tests/ui/server_send/error/has-too-many-arguments.rs
    //     abort_call_site!(ERROR; help = HELP)
    // }

    proc_macro::TokenStream::from(quote::quote! {fn dummy(){}})
}

pub fn parse(metadata: TokenStream, item: TokenStream) -> Ast {

    parse_args(metadata.clone().into());

    // let expr = syn::parse::<Expr>(metadata.clone().into()).expect("Expr parsing");
    // match expr {
    //     Expr::Array(expr) => {
    //         // ../tests/ui/server_send/error/has-expression-arguments.rs
    //         abort!(expr, ERROR; help = HELP)
    //     }
    //     Expr::Assign(_expr) => {
    //         parse_args(metadata.into());
    //     }
    //     _ => {
    //         // ../tests/ui/server_send/error/has-other-arguments.rs
    //         abort!(expr, ERROR; help = HELP)
    //     }
    // }

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
//     use proc_macro_error::proc_macro_error;

//     use super::*;

    // #[test]
    // // #[proc_macro_attribute]
    // // #[proc_macro_error(allow_not_macro)]
    // fn valid_minimal_syntax() {
    //     parse_args(
    //         quote!(
    //             #[inline]
    //             #[server_send(level = info)]
    //             fn even_to_odd(x: u32) -> u32 {
    //                 x + 1
    //             }
    //         ).into()
    //     );
    //     //proc_macro::TokenStream::from(quote::quote!{fn dummy(){}})
    // }

//     #[test]
//     #[proc_macro_attribute]
//     //#[proc_macro_error(allow_not_macro)]
//     fn valid_instrument_syntax() {
//         parse_args(
//             quote!(
//                 #[inline]
//                 #[server_send(level = trace, name = "Neat", skip = "x,y")]
//                 fn some_addition(x: u32, y: u32) -> u32 {
//                     x + y
//                 }
//             ).into()
//         );
//         //proc_macro::TokenStream::from(quote::quote!{fn dummy(){}})
//     }
// }

// Args { vars: {ExprAssign { attrs: [], left: Path(ExprPath { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "target", span: #0 bytes(94..100) }, arguments: None }] } }), eq_token: Eq, right: Lit(ExprLit { attrs: [], lit: Str(LitStr { token: "bulls-eye" }) }) }, ExprAssign { attrs: [], left: Path(ExprPath { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "level", span: #0 bytes(58..63) }, arguments: None }] } }), eq_token: Eq, right: Path(ExprPath { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "trace", span: #0bytes(64..69) }, arguments: None }] } }) }, ExprAssign { attrs: [], left: Path(ExprPath { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "name", span: #0 bytes(71..75) }, arguments: None }] } }), eq_token: Eq, right: Lit(ExprLit { attrs: [], lit: Str(LitStr { token: "B" }) }) }, ExprAssign { attrs: [], left: Path(ExprPath { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "skip", span: #0 bytes(81..85) }, arguments: None }] } }), eq_token: Eq, right: Array(ExprArray { attrs: [], bracket_token: Bracket, elems: [Path(ExprPath { attrs: [], qself: None, path: Path { leading_colon: None, segments: [PathSegment { ident: Ident { ident: "self", span: #0 bytes(87..91) }, arguments: None }] } })] }) }} }