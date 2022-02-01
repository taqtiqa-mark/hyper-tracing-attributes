use proc_macro_error::{abort, abort_call_site};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    ItemFn,
};

use crate::server_send::Ast;

pub fn analyze(ast: Ast) -> Model {
    let mut trace_fields = vec![];

    let mut item = ast;
    let attrs = &mut item.attrs;

    eprintln!("{:?}", attrs);

    for index in (0..attrs.len()).rev() {
        if let Some(ident) = attrs[index].path.get_ident() {
            if ident.to_string().as_str() == "trace_field" {
                let attr = attrs.remove(index);
                let span = attr.tokens.span();

                if let Ok(arg) = syn::parse2::<AttributeArgument>(attr.tokens) {
                    trace_fields.push(arg.expr);
                } else {
                    // ../tests/ui/server_send/error/trace_field-is-not-an-assignment-expression.rs
                    abort!(
                        span,
                        "expected an assignment expression as argument";
                        help = "example: `#[trace_field(b = tracing::field::Empty)]`")
                }
            }
        }
    }

    // if trace_fields.is_empty() {
    //     // ../tests/ui/server_send/error/zero-trace-feilds.rs
    //     abort_call_site!(
    //         "no trace fields were specified";
    //         help = "add a `#[trace_field]`"
    //     )
    // }

    Model { trace_fields, item }
}

struct AttributeArgument {
    expr: syn::ExprAssign,
}

impl Parse for AttributeArgument {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let _parenthesis = parenthesized!(content in input);

        Ok(AttributeArgument {
            expr: content.parse()?,
        })
    }
}

pub struct Model {
    pub trace_fields: Vec<syn::ExprAssign>,
    pub item: ItemFn,
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, Attribute};

    use super::*;

    #[test]
    fn can_extract_trace_field() {
        let model = analyze(parse_quote!(
            #[trace_field(x=0)]
            fn f(x: bool) {}
        ));

        let expected: &[syn::ExprAssign] = &[parse_quote!(x=0)];
        assert_eq!(expected, model.trace_fields);

        assert!(model.item.attrs.is_empty());
    }

    // Ensure attributes unrelated to this proc-macro are not removed or reordered
    #[test]
    fn non_dsl_attributes_are_preserved() {
        let model = analyze(parse_quote!(
            #[a]
            #[trace_field(x=1)]
            #[b]
            fn f(x: bool) {}
        ));

        let expected: &[Attribute] = &[parse_quote!(#[a]), parse_quote!(#[b])];
        assert_eq!(expected, model.item.attrs);
    }
}
