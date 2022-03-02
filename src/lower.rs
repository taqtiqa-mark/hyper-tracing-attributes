use quote::quote;
use syn::{Expr, ItemFn};

use crate::Model;

pub fn lower(model: Model) -> Ir {
    let Model {
        trace_fields,
        item,
    } = model;

    let assertions = trace_fields
        .into_iter()
        .map(|expr| Assertion {
            message: format!("violation of trace_field `{}`", quote!(#expr)),
            expr,
        })
        .collect();

    Ir { assertions, item }
}

pub struct Ir {
    pub assertions: Vec<Assertion>,
    pub item: ItemFn,
}

pub struct Assertion {
    pub expr: Expr,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    impl Model {
        fn stub() -> Self {
            Self {
                trace_fields: vec![],
                item: parse_quote!(
                    fn f() {}
                ),
            }
        }
    }

    #[test]
    fn produces_assertion_for_trace_field() {
        let mut model = Model::stub();
        model.trace_fields.push(parse_quote!(x));

        let ir = lower(model);

        assert_eq!(1, ir.assertions.len());

        let assertion = &ir.assertions[0];
        let expected: Expr = parse_quote!(x);
        assert_eq!(expected, assertion.expr);
        assert_eq!("violation of trace_field `x`", assertion.message);
    }
}
