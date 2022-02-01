use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use syn::{Expr, Item, ItemFn};

pub type Ast = ItemFn;

// Attribute arguments that are assignment expressions.
pub struct Args {
    vars: std::collections::HashSet<syn::ExprAssign>
}

// Parse the attribute data once we know it contains ExprAssign
impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        // parses a=1,b="d",c=f(), where a,b and c are ExprAssign
        let vars = syn::punctuated::Punctuated::<syn::ExprAssign, syn::Token![,]>::parse_terminated(input)?;
        Ok(Args {
            vars: vars.into_iter().collect(),
        })
    }
}

// Using newly created struct Args, parse assignment arguments then count.
// This utility/no-op function is required by the fact
// `syn::parse_macro_input!` must be called in a function that returns
// proc_macro::TokenStream.
pub fn parse_args(metadata: proc_macro::TokenStream) -> proc_macro::TokenStream {
    const ERROR: &str = "this attribute takes one, two or three arguments";
    const HELP: &str = "use `#[server_send(level=trace, name=\"Name\", skip=[a,b])]`";

    eprintln!("{}", metadata);
    let expr = syn::parse2::<syn::ExprAssign>(metadata.clone().into()).expect("ExprAssign parsing");
    let args = syn::parse_macro_input!(metadata as Args);

    if args.vars.len() > 3 {
        // ../tests/ui/server_send/error/has-too-many-arguments.rs
        abort!(expr, ERROR; help = HELP)
    }
    proc_macro::TokenStream::from(quote::quote!{fn dummy(){}})
}

pub fn parse(metadata: TokenStream, item: TokenStream) -> Ast {
    const ERROR: &str = "this attribute takes one, two or three arguments";
    const HELP: &str = "use `#[server_send(level=trace, name=\"Name\", skip=[a,b])]`";

    if metadata.is_empty() {
        // ../tests/ui/server_send/error/has-no-arguments.rs
        abort_call_site!(ERROR; help = HELP)
    }

    let expr = syn::parse2::<Expr>(metadata.clone()).expect("Expr parsing");
    match expr {
        Expr::Array(expr) => {
            // ../tests/ui/server_send/error/has-expression-arguments.rs
            abort!(expr, ERROR; help = HELP)
        }
        Expr::Assign(_expr) => {
            parse_args(metadata.into());
        }
        _ => {
            // ../tests/ui/server_send/error/has-other-arguments.rs
            abort!(expr, ERROR; help = HELP)
        }
    }


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

#[cfg(test)]
mod tests {
    use quote::quote;

    use super::*;

    #[test]
    fn valid_field_syntax() {
        parse(
            quote!(),
            quote!(
                #[inline]
                #[trace_field(x = 0)]
                fn even_to_odd(x: u32) -> u32 {
                    x + 1
                }
            ),
        );
    }
    #[test]
    fn valid_instrument_syntax() {
        parse(
            quote!(),
            quote!(
                #[inline]
                #[server_send(level = trace, name = "Neat", skip = [x])]
                fn even_to_odd(x: u32) -> u32 {
                    x + 1
                }
            ),
        );
    }
}
