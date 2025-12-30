// Generated macro for expr_for_container_with (function)
macro_rules! Depcrate_schema_exprsexpr_for_container_with {
() => {
// Module: crate::schema_exprs
// Provides: {"expr_for_container_with"}
// Dependencies: {}
fn expr_for_container_with (cont : & Container , with_attr : & WithAttr) -> SchemaExpr { let (ty , type_def) = type_for_schema (cont , with_attr) ; let mut schema_expr = SchemaExpr :: from (quote ! { <# ty as schemars :: JsonSchema >:: json_schema (# GENERATOR) }) ; schema_expr . definitions . extend (type_def) ; schema_expr }
};
}
