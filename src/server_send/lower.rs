use quote::quote;
use syn::{ItemFn};

use crate::server_send::Model;

pub fn lower(model: Model) -> Ir {
    let Model { trace_fields, item } = model;

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
    pub expr: syn::ExprAssign,
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
        model.trace_fields.push(parse_quote!(x = 2));

        let ir = lower(model);

        assert_eq!(1, ir.fields.len());

        let field = &ir.fields[0];
        let expected: syn::ExprAssign = parse_quote!(x=2);
        assert_eq!(expected, field.expr);
        assert_eq!("violation of trace_field `x = 2`", field.message);
    }
}
