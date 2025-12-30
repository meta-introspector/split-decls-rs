// Generated macro for expr_for_enum (function)
macro_rules! Depcrate_schema_exprsexpr_for_enum {
() => {
// Module: crate::schema_exprs
// Provides: {"expr_for_enum"}
// Dependencies: {}
fn expr_for_enum (cont : & Container , variants : & [Variant] , cattrs : & serde_attr :: Container ,) -> SchemaExpr { if variants . is_empty () { return quote ! (schemars :: Schema :: from (false)) . into () ; } let deny_unknown_fields = cattrs . deny_unknown_fields () ; let variants = variants . iter () ; match cattrs . tag () { TagType :: External => expr_for_external_tagged_enum (cont , variants , deny_unknown_fields) , TagType :: None => expr_for_untagged_enum (cont , variants , deny_unknown_fields) , TagType :: Internal { tag } => { expr_for_internal_tagged_enum (cont , variants , tag , deny_unknown_fields) } TagType :: Adjacent { tag , content } => { expr_for_adjacent_tagged_enum (cont , variants , tag , content , deny_unknown_fields) } } }
};
}
