// Generated macro for expr_for_internal_tagged_enum_variant (function)
macro_rules! Depcrate_schema_exprsexpr_for_internal_tagged_enum_variant {
() => {
// Module: crate::schema_exprs
// Provides: {"expr_for_internal_tagged_enum_variant"}
// Dependencies: {}
fn expr_for_internal_tagged_enum_variant (cont : & Container , variant : & Variant , deny_unknown_fields : bool ,) -> SchemaExpr { if let Some (with_attr) = & variant . attrs . with { let (ty , type_def) = type_for_schema (cont , with_attr) ; let mut schema_expr = SchemaExpr :: from (quote_spanned ! { variant . original . span () => <# ty as schemars :: JsonSchema >:: json_schema (# GENERATOR) }) ; schema_expr . definitions . extend (type_def) ; return schema_expr ; } match variant . style { Style :: Unit => expr_for_unit_struct () , Style :: Newtype => expr_for_field (cont , & variant . fields [0] , true) , Style :: Tuple => expr_for_tuple_struct (cont , & variant . fields) , Style :: Struct => expr_for_struct (cont , & variant . fields , & SerdeDefault :: None , deny_unknown_fields ,) , } }
};
}
