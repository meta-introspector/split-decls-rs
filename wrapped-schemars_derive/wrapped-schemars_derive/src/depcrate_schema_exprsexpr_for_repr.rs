// Generated macro for expr_for_repr (function)
macro_rules! Depcrate_schema_exprsexpr_for_repr {
() => {
// Module: crate::schema_exprs
// Provides: {"expr_for_repr"}
// Dependencies: {}
pub fn expr_for_repr (cont : & Container) -> Result < SchemaExpr , syn :: Error > { let repr_type = cont . attrs . repr . as_ref () . ok_or_else (| | { syn :: Error :: new (Span :: call_site () , "JsonSchema_repr: missing #[repr(...)] attribute" ,) }) ? ; let Data :: Enum (variants) = & cont . data else { return Err (syn :: Error :: new (Span :: call_site () , "JsonSchema_repr can only be used on enums" ,)) ; } ; if let Some (non_unit_error) = variants . iter () . find_map (| v | match v . style { Style :: Unit => None , _ => Some (syn :: Error :: new_spanned (v . original , "JsonSchema_repr: must be a unit variant" ,)) , }) { return Err (non_unit_error) ; } let enum_ident = & cont . ident ; let variant_idents = variants . iter () . map (| v | & v . ident) ; let mut schema_expr = SchemaExpr :: from (quote ! ({ let mut map = schemars :: _private :: serde_json :: Map :: new () ; map . insert ("type" . into () , "integer" . into ()) ; map . insert ("enum" . into () , schemars :: _private :: serde_json :: Value :: Array ({ let mut enum_values = schemars :: _private :: alloc :: vec :: Vec :: new () ; # (enum_values . push ((# enum_ident ::# variant_idents as # repr_type) . into ()) ;) * enum_values }) ,) ; schemars :: Schema :: from (map) })) ; cont . add_mutators (& mut schema_expr . mutators) ; Ok (schema_expr) }
};
}
