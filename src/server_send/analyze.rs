use proc_macro_error::abort;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    spanned::Spanned,
    ItemFn,
};

use crate::server_send::parse::Args;
use crate::server_send::Ast;

pub fn analyze(ast: Ast, meta: Args) -> Model {
    let mut fields = vec![];
    let mut level: syn::ExprPath = syn::parse_quote!(Level::DEBUG);
    let mut name: syn::ExprLit = syn::parse_quote!("server_end");
    let mut skip: syn::ExprArray = syn::parse_quote!([]);

    let mut item = ast;
    let attrs = &mut item.attrs;

    for index in (0..attrs.len()).rev() {
        if let Some(ident) = attrs[index].path.get_ident() {
            let id = ident.to_string();
            if id.as_str() == "trace_field" {
                let attr = attrs.remove(index);
                let span = attr.tokens.span();
                get_field(&mut fields, span, attr.tokens);
            }
        }
    }

    for expr in meta.vars {
        //eprintln!("Meta argument: {:#?}", expr);
        #[allow(clippy::single_match)]
        match expr {
            syn::Expr::Assign(e) => {
                //eprintln!("Meta Expr::Assign: left: {:#?} right: {:#?}", *e.left, *e.right);
                if let syn::Expr::Path(ref el) = *e.left {
                        let first = el.path.segments.first().unwrap();
                        if first.arguments.is_empty() {
                            let param = first.ident.to_string();
                            //eprintln!("Meta Expr::Assign: left: {:#?}", param);
                            match param.as_str() {
                                "skip" => {
                                    if let syn::Expr::Array(er) = *e.right { skip = er;}
                                }
                                "name" => {
                                    if let syn::Expr::Lit(er) = *e.right { name = er;}
                                }
                                "level" => {
                                    if let syn::Expr::Path(er) = *e.right { level = er;}
                                }
                                &_ => {}
                            }
                        }
                    };

            }
            // syn::Expr::Path(ref expr) => {
            //     if expr.path.leading_colon.is_some() {
            //         //eprintln!("Meta Expr::Path: false");
            //     } else if expr.path.segments.len() != 1 {
            //         //eprintln!("Meta Expr::Path: {:?}",expr.path.segments);
            //     } else {
            //         //let first = expr.path.segments.first().unwrap();
            //         //self.vars.contains(&first.ident) && first.arguments.is_empty()
            //         //eprintln!("Meta Expr::Path: {:?}",first.arguments);
            //     }
            // }
            _ => {}
        }
    }

    Model {
        fields,
        item,
        level,
        name,
        skip,
    }
}

fn get_field(
    fields: &mut Vec<syn::ExprAssign>,
    span: proc_macro2::Span,
    tokens: proc_macro2::TokenStream,
) {
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
    pub name: syn::ExprLit,
    pub skip: syn::ExprArray,
}

#[cfg(test)]
mod tests {
    use syn::{parse_quote, Attribute};

    use super::*;

    #[test]
    fn can_extract_trace_field() {
        let meta_args = Args { vars: vec![]} ;

        let model = analyze(
            parse_quote!(
                #[trace_field(x = 0)]
                fn f(x: bool) {}
            ),
            meta_args,
        );

        let expected: &[syn::ExprAssign] = &[parse_quote!(x = 0)];
        assert_eq!(expected, model.fields);

        assert!(model.item.attrs.is_empty());
    }

    // #[test]
    // fn can_extract_level() {
    //     let meta_args = Args { vars: vec![]} ;
    //     let model = analyze(
    //         parse_quote!(
    //             #[server_send(level=Level::DEBUG)]
    //             fn f(x: bool) {}
    //         ),
    //         parse_quote!(
    //             #[server_send(level=Level::DEBUG)]
    //         ),
    //     );

    //     let expected: syn::ExprPath = parse_quote!(Level::DEBUG);
    //     assert_eq!(expected, model.level);

    //     // assert!(model.item.attrs.is_empty());
    // }

    // #[test]
    // fn can_extract_skip() {
    //     let model = analyze(
    //         parse_quote!(
    //             #[server_send(skip=[x])]
    //             fn f(x: bool) {}
    //         ),
    //         parse_quote!(
    //             #[server_send(skip=[x])]
    //         ),
    //     );

    //     let expected: syn::ExprArray = parse_quote!([x]);
    //     assert_eq!(expected, model.skip);

    //     // assert!(model.item.attrs.is_empty());
    // }

    // Ensure attributes unrelated to this proc-macro are not removed or reordered
    #[test]
    fn non_dsl_attributes_are_preserved() {

        // let _ts = crate::utility::to_token_stream("#[server_send(level=tracing::Level::TRACE)]");
        // eprintln!("Token Stream{:#?}",ts);


        let model = analyze(
            parse_quote!(
                #[a]
                #[trace_field(x = 0)]
                #[b]
                fn f(x: bool) {}
            ),
            parse_quote!(
                // #[server_send(level=tracing::Level::TRACE)]
            ),
        );

        let expected: &[Attribute] = &[parse_quote!(#[a]), parse_quote!(#[b])];
        assert_eq!(expected, model.item.attrs);
    }
}
