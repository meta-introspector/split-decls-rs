// Generated macro for expr_for_untagged_enum (function)
macro_rules! Depcrate_schema_exprsexpr_for_untagged_enum {
() => {
// Module: crate::schema_exprs
// Provides: {"expr_for_untagged_enum"}
// Dependencies: {}
fn expr_for_untagged_enum < 'a > (cont : & Container , variants : impl Iterator < Item = & 'a Variant < 'a > > , deny_unknown_fields : bool ,) -> SchemaExpr { let schemas = variants . map (| variant | { let schema_expr = expr_for_untagged_enum_variant (cont , variant , deny_unknown_fields , true) ; (Some (variant) , schema_expr) }) . collect () ; variant_subschemas (cont , false , schemas) }
};
}
