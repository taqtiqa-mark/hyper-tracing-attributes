use quote::quote;
use syn::{Expr, ItemFn};

use crate::Model;

pub fn lower(model: Model) -> Ir {
    let Model {
        trace_fields,
        item,
    } = model;

    let fields = trace_fields
        .into_iter()
        .map(|expr| Field {
            message: format!("violation of trace_field `{}`", quote!(#expr)),
            expr,
        })
        .collect();

    Ir { fields, item }
}

pub struct Ir {
    pub fields: Vec<Field>,
    pub item: ItemFn,
}

pub struct Field {
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

        assert_eq!(1, ir.fields.len());

        let assertion = &ir.fields[0];
        let expected: Expr = parse_quote!(x);
        assert_eq!(expected, assertion.expr);
        assert_eq!("violation of trace_field `x`", assertion.message);
    }
}
