// Generated macro for SchemaExpr (struct)
macro_rules! Depcrate_schema_exprsSchemaExpr {
() => {
// Module: crate::schema_exprs
// Provides: {"SchemaExpr"}
// Dependencies: {}
pub struct SchemaExpr { # [doc = " Definitions for types or functions that may be used within the creator or mutators"] definitions : Vec < TokenStream > , # [doc = " An expression that produces a `Schema`"] creator : TokenStream , # [doc = " Statements (including terminating semicolon) that mutate a var `schema` of type `Schema`"] mutators : Vec < TokenStream > , }
};
}
