use syn::{ItemFn};

use crate::server_send::Model;

pub fn lower(model: Model) -> Ir {
    let Model {
        level,
        fields,
        item,
    } = model;

    let fields = fields
        .into_iter()
        .map(|expr| Field {
            expr,
        })
        .collect();

    let level = level;

    Ir { level, fields, item }
}

#[derive(Debug)]
pub struct Ir {
    pub level: syn::ExprPath,
    pub fields: Vec<Field>,
    pub item: ItemFn,
}

#[derive(Debug)]
pub struct Field {
    pub expr: syn::ExprAssign,
    //pub message: String,
}

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    impl Model {
        fn stub() -> Self {
            Self {
                level: parse_quote!(Level::TRACE),
                fields: vec![],
                item: parse_quote!(
                    fn f() {}
                ),
            }
        }
    }

    #[test]
    fn produces_expression_for_field() {
        let mut model = Model::stub();
        model.fields.push(parse_quote!(x=0));

        let ir = lower(model);

        assert_eq!(1, ir.fields.len());

        let field = &ir.fields[0];
        let expected: syn::ExprAssign = parse_quote!(x=0);
        assert_eq!(expected, field.expr);
    }

    #[test]
    fn produces_expression_for_level() {
        // Setup Level::TRACE
        let mut model = Model::stub();
        model.level = parse_quote!(Level::DEBUG);

        let ir = lower(model);

        let level = ir.level;
        let expected: syn::ExprPath = parse_quote!(Level::DEBUG);
        assert_eq!(expected, level);
    }
}
