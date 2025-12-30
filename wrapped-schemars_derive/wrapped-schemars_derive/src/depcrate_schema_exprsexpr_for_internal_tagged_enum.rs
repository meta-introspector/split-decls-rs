// Generated macro for expr_for_internal_tagged_enum (function)
macro_rules! Depcrate_schema_exprsexpr_for_internal_tagged_enum {
() => {
// Module: crate::schema_exprs
// Provides: {"expr_for_internal_tagged_enum"}
// Dependencies: {}
fn expr_for_internal_tagged_enum < 'a > (cont : & Container , variants : impl Iterator < Item = & 'a Variant < 'a > > , tag_name : & str , deny_unknown_fields : bool ,) -> SchemaExpr { let variant_schemas = variants . map (| variant | { if variant . serde_attrs . untagged () { return (Some (variant) , expr_for_untagged_enum_variant (cont , variant , deny_unknown_fields , true)) } let mut schema_expr = expr_for_internal_tagged_enum_variant (cont , variant , deny_unknown_fields) ; let name = variant . name () ; schema_expr . mutators . push (quote ! (schemars :: _private :: apply_internal_enum_variant_tag (& mut # SCHEMA , # tag_name , # name , # deny_unknown_fields) ;)) ; variant . add_mutators (& mut schema_expr . mutators) ; (Some (variant) , schema_expr) }) . collect () ; variant_subschemas (cont , true , variant_schemas) }
};
}
