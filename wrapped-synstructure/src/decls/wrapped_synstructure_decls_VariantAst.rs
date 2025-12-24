use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// This type is similar to `syn`'s `Variant` type, however each of the fields
/// are references rather than owned. When this is used as the AST for a real
/// variant, this struct simply borrows the fields of the `syn::Variant`,
/// however this type may also be used as the sole variant for a struct.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct VariantAst<'a> {
    pub attrs: &'a [Attribute],
    pub ident: &'a Ident,
    pub fields: &'a Fields,
    pub discriminant: &'a Option<(token::Eq, Expr)>,
}
