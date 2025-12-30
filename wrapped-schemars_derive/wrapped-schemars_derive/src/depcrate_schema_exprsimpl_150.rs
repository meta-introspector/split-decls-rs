// Generated macro for impl_150 (impl)
macro_rules! Depcrate_schema_exprsimpl_150 {
() => {
// Module: crate::schema_exprs
// Provides: {"impl_150"}
// Dependencies: {}
impl ToTokens for SchemaExpr { fn to_tokens (& self , tokens : & mut TokenStream) { let Self { definitions , creator , mutators , } = self ; tokens . extend (if mutators . is_empty () { quote ! ({ # (# definitions) * # creator }) } else { quote ! ({ # (# definitions) * let mut # SCHEMA = # creator ; # (# mutators) * # SCHEMA }) }) ; } }
};
}
