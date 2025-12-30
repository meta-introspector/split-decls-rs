// Generated macro for expr_for_tuple_struct (function)
macro_rules! Depcrate_schema_exprsexpr_for_tuple_struct {
() => {
// Module: crate::schema_exprs
// Provides: {"expr_for_tuple_struct"}
// Dependencies: {}
fn expr_for_tuple_struct (cont : & Container , fields : & [Field]) -> SchemaExpr { let fields : Vec < _ > = fields . iter () . map (| f | { let field_expr = expr_for_field (cont , f , false) ; f . with_contract_check (quote ! { prefix_items . push ((# field_expr) . to_value ()) ; }) }) . collect () ; let max_len = fields . len () ; quote ! ({ let mut prefix_items = schemars :: _private :: alloc :: vec :: Vec ::< schemars :: _private :: serde_json :: Value >:: with_capacity (# max_len) ; # (# fields) * let len = prefix_items . len () ; let mut map = schemars :: _private :: serde_json :: Map :: new () ; map . insert ("type" . into () , "array" . into ()) ; if ! prefix_items . is_empty () { map . insert ("prefixItems" . into () , prefix_items . into ()) ; map . insert ("minItems" . into () , len . into ()) ; } map . insert ("maxItems" . into () , len . into ()) ; schemars :: Schema :: from (map) }) . into () }
};
}
