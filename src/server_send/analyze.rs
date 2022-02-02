use proc_macro_error::{abort};
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    ItemFn,
};

use crate::server_send::Ast;

pub fn analyze(ast: Ast) -> Model {
    let mut fields = vec![];
    let mut level: syn::ExprPath = syn::parse_quote!(Level::DEBUG);
    let mut skip: syn::ExprArray = syn::parse_quote!([]);

    let mut item = ast;
    let attrs = &mut item.attrs;
    for index in (0..attrs.len()).rev() {
        eprintln!("{:#?}", attrs[index]);
        if let Some(ident) = attrs[index].path.get_ident() {
            let id = ident.to_string();
            match id.as_str() {
                "server_send" | "trace_field" => {
                    let attr = attrs.remove(index);
                    let span = attr.tokens.span();
                    if id == "server_send" {
                        get_level(&mut level, span, attr.tokens.clone());
                        get_skip(&mut skip, span, attr.tokens.clone());
                    }
                    if id == "trace_field" {
                        get_field(&mut fields, span, attr.tokens);
                    }
                }
                _ => {}
            }
        }
    }

    Model {
        fields,
        item,
        level,
        skip,
    }
}

fn get_field(fields: &mut Vec<syn::ExprAssign>, span: proc_macro2::Span, tokens: proc_macro2::TokenStream) {
    if let Ok(arg) = syn::parse2::<AttributeArgument>(tokens) {
        fields.push(arg.expr);
    } else {
        // ../tests/ui/server_send/error/trace_field-is-not-an-assignment-expression.rs
        abort!(
            span,
            "expected an assigned expression as argument";
            help = "example: `#[trace_field(argument = 0)]`")
    }
}

fn get_level(level: &mut syn::ExprPath, span: proc_macro2::Span, tokens: proc_macro2::TokenStream) {

    if let Ok(arg) = syn::parse2::<AttributeArgument>(tokens) {
        if let syn::Expr::Path(path) = *(arg.expr).right {
            *level = path;
        }
    } else {
        // ../tests/ui/server_send/error/trace_field-is-not-an-assignment-expression.rs
        abort!(
            span,
            "expected an assigned expression as argument";
            help = "example: `#[server_send(level = tracing::Level::INFO)]`")
    }
}

fn get_skip(skip: &mut syn::ExprArray, span: proc_macro2::Span, tokens: proc_macro2::TokenStream) {

    if let Ok(arg) = syn::parse2::<AttributeArgument>(tokens) {
        if let syn::Expr::Array(ary) = *(arg.expr).right {
            *skip = ary;
        }
    } else {
        // ../tests/ui/server_send/error/trace_field-is-not-an-assignment-expression.rs
        abort!(
            span,
            "expected an assigned expression as argument";
            help = "example: `#[server_send(skip = [a,b,c])]`")
    }
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

#[derive(Debug)]
pub struct Model {
    pub fields: Vec<syn::ExprAssign>,
    pub item: ItemFn,
    pub level: syn::ExprPath,
    pub skip: syn::ExprArray,
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
        assert_eq!(expected, model.fields);

        assert!(model.item.attrs.is_empty());
    }

    #[test]
    fn can_extract_level() {
        let model = analyze(parse_quote!(
            #[server_send(level=Level::DEBUG)]
            fn f(x: bool) {}
        ));

        let expected: syn::ExprPath = parse_quote!(Level::DEBUG);
        assert_eq!(expected, model.level);

        assert!(model.item.attrs.is_empty());
    }

    #[test]
    fn can_extract_skip() {
        let model = analyze(parse_quote!(
            #[server_send(skip=[x])]
            fn f(x: bool) {}
        ));

        let expected: syn::ExprArray = parse_quote!([x]);
        assert_eq!(expected, model.skip);

        assert!(model.item.attrs.is_empty());
    }

    // Ensure attributes unrelated to this proc-macro are not removed or reordered
    #[test]
    fn non_dsl_attributes_are_preserved() {
        let model = analyze(parse_quote!(
            #[a]
            #[trace_field(x=0)]
            #[b]
            fn f(x: bool) {}
        ));

        let expected: &[Attribute] = &[parse_quote!(#[a]), parse_quote!(#[b])];
        assert_eq!(expected, model.item.attrs);
    }
}
