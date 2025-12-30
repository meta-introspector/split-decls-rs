// Generated macro for expr_for_untagged_enum_variant (function)
macro_rules! Depcrate_schema_exprsexpr_for_untagged_enum_variant {
() => {
// Module: crate::schema_exprs
// Provides: {"expr_for_untagged_enum_variant"}
// Dependencies: {}
fn expr_for_untagged_enum_variant (cont : & Container , variant : & Variant , deny_unknown_fields : bool , is_actually_untagged : bool ,) -> SchemaExpr { let mut schema_expr = if let Some (with_attr) = & variant . attrs . with { let (ty , type_def) = type_for_schema (cont , with_attr) ; let mut schema_expr = SchemaExpr :: from (quote_spanned ! { variant . original . span () => # GENERATOR . subschema_for ::<# ty > () }) ; schema_expr . definitions . extend (type_def) ; schema_expr } else { match variant . style { Style :: Unit => expr_for_unit_struct () , Style :: Newtype => expr_for_field (cont , & variant . fields [0] , false) , Style :: Tuple => expr_for_tuple_struct (cont , & variant . fields) , Style :: Struct => expr_for_struct (cont , & variant . fields , & SerdeDefault :: None , deny_unknown_fields ,) , } } ; if is_actually_untagged { if variant . attrs . common . title . is_none () { let title = variant . name () ; schema_expr . mutators . push (quote ! { if # GENERATOR . settings () . untagged_enum_variant_titles { # SCHEMA . insert ("title" . into () , # title . into ()) ; } }) ; } variant . add_mutators (& mut schema_expr . mutators) ; } schema_expr }
};
}
