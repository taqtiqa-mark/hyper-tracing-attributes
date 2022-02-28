#![allow(unused_variables)]

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, ItemFn, Stmt};

use crate::server_send::{lower::Field, Ir};

pub type Rust = TokenStream;

pub fn codegen(ir: Ir) -> Rust {
    let Ir {
        fields,
        item,
        level,
        name,
        skip,
    } = ir;

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = item;

    let flds: Vec<syn::ExprAssign> = fields.into_iter().map(|p| p.expr).collect();
    let stmts = &block.stmts;

    // Inspect statements for a TraceableError path.

    // When no skip values are given, do not insert `skip(...)`
    let mut skip_tokens = quote::quote!();
    if !skip.elems.is_empty() {
        let skp: Vec<syn::Expr> = skip.elems.into_iter().map(|s| s).collect();
        skip_tokens = quote::quote!(
            skip(#(#skp),*),
        );
    }
    let code = syn::parse_quote! {
        #(#attrs)*
        #[cfg_attr(feature = "traceable",
            tracing::instrument(level = #level,
                                // Let tracing create the default name.
                                #skip_tokens
                                fields( // Custom via `trace_field(...)`
                                        #(#flds ,)*
                                        #[cfg(feature = "otel")]
                                        {
                                            // Standardized (OpenTelemetry)
                                            otel.name           = #name,
                                            // otel.kind           = ?opentelemetry::trace::SpanKind::Server,
                                            // otel.status_code    = ?opentelemetry::trace::StatusCode::Unset,
                                            otel.status_message = tracing::field::Empty,
                                            otel.library.name   = "tracing-attributes-http",
                                        }
                                        #[cfg(any(feature = "http",feature = "http-full"))]
                                        {
                                            // OTel-HTTP required at span creation
                                            http.method   = tracing::field::Empty,
                                            http.scheme   = tracing::field::Empty,
                                            http.target   = tracing::field::Empty,
                                            http.url      = tracing::field::Empty,
                                            net.host.name = tracing::field::Empty,
                                            net.host.port = tracing::field::Empty,
                                        }
                                        #[cfg(feature = "http-full")]
                                        {
                                            // OTel-HTTP optional at span creation
                                            http.flavor                               = tracing::field::Empty,
                                            http.response_content_length_uncompressed = tracing::field::Empty,
                                            http.response_content_length              = tracing::field::Empty,
                                            http.response.header.content_type         = tracing::field::Empty,
                                            http.status_code                          = tracing::field::Empty,
                                            http.user_agent                           = tracing::field::Empty,
                                            net.peer.ip                               = tracing::field::Empty,
                                            net.peer.name                             = tracing::field::Empty,
                                            net.peer.port                             = tracing::field::Empty,
                                            // OTel-HTTP server semantics
                                            http.server_name = tracing::field::Empty,
                                            http.route       = tracing::field::Empty,
                                            http.client_ip   = tracing::field::Empty,
                                            // OTel-General optional
                                            net.transport = "IP.TCP",
                                        }
                                )
                        )
                    )]
        #vis #sig {
            #(#stmts)*
        }
    };
    code
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Field { expr } = self;
        // The trace field line: x = 0,
        let stmt: Stmt = parse_quote!(#expr);
        stmt.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_is_function_item() {
        let pq: syn::ExprAssign = parse_quote!(x = 0);
        let i = syn::parse2(
            quote::quote!(
                fn f() {}
            )
            .into(),
        )
        .expect("ItemFn");
        let f = vec![Field { expr: pq }];
        let l = syn::parse_quote!(Level::TRACE);
        let n = syn::parse_quote!("some_test");
        let s = syn::parse_quote!([a, b, c]);
        let ir = Ir {
            fields: f,
            item: i,
            level: l,
            name: n,
            skip: s,
        };

        let rust = codegen(ir);

        assert!(syn::parse2::<ItemFn>(rust).is_ok());
    }
}
