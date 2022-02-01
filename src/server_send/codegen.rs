use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, ItemFn, Stmt};

use crate::server_send::{lower::Field, Ir};

pub type Rust = TokenStream;

pub fn codegen(ir: Ir) -> Rust {
    let Ir { fields, item } = ir;

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = item;

    // https://github.com/rust-lang/rust/issues/17022
    quote!(
        #(#attrs)*
        #(#fields)*
        #vis #sig {
            #block
        }
    )
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Field { expr, message } = self;
        let stmt: Stmt = parse_quote!(instrument(#expr, #message););
        stmt.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_function_item() {
        let ir = Ir {
            fields: vec![Field {
                expr: parse_quote!(x=0),
                message: "message".to_string(),
            }],
            item: parse_quote!(
                fn f() {}
            ),
        };
        let rust = codegen(ir);

        eprintln!("{}", rust);

        assert!(syn::parse2::<ItemFn>(rust).is_ok());
    }
}
