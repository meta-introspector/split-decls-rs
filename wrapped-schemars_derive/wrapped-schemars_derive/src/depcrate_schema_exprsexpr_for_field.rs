// Generated macro for expr_for_field (function)
macro_rules! Depcrate_schema_exprsexpr_for_field {
() => {
// Module: crate::schema_exprs
// Provides: {"expr_for_field"}
// Dependencies: {}
fn expr_for_field (cont : & Container , field : & Field , is_internal_tagged_enum_newtype : bool ,) -> SchemaExpr { let (ty , type_def) = type_for_field_schema (cont , field) ; let span = field . original . span () ; let schema_expr = if field . attrs . validation . required { quote_spanned ! { span => <# ty as schemars :: JsonSchema >:: _schemars_private_non_optional_json_schema (# GENERATOR) } } else if is_internal_tagged_enum_newtype { quote_spanned ! { span => schemars :: _private :: json_schema_for_internally_tagged_enum_newtype_variant ::<# ty > (# GENERATOR) } } else { quote_spanned ! { span => # GENERATOR . subschema_for ::<# ty > () } } ; let mut schema_expr = SchemaExpr :: from (schema_expr) ; schema_expr . definitions . extend (type_def) ; field . add_mutators (& mut schema_expr . mutators) ; schema_expr }
};
}
