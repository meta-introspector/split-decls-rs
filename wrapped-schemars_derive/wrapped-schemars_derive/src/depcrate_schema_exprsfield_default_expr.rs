// Generated macro for field_default_expr (function)
macro_rules! Depcrate_schema_exprsfield_default_expr {
() => {
// Module: crate::schema_exprs
// Provides: {"field_default_expr"}
// Dependencies: {}
fn field_default_expr (field : & Field , container_has_default : bool) -> Option < TokenStream > { let field_default = field . serde_attrs . default () ; if field . serde_attrs . skip_serializing () || (field_default . is_none () && ! container_has_default) { return None ; } let ty = field . ty ; let default_expr = match field_default { SerdeDefault :: None => { let member = & field . member ; quote ! (# STRUCT_DEFAULT .# member) } SerdeDefault :: Default => quote ! (<# ty >:: default ()) , SerdeDefault :: Path (path) => quote ! (# path ()) , } ; let default_expr = if let Some (skip_if) = field . serde_attrs . skip_serializing_if () { quote ! { { let default = # default_expr ; if # skip_if (& default) { None } else { Some (default) } } } } else { quote ! (Some (# default_expr)) } ; Some (if let Some (ser_with) = field . serde_attrs . serialize_with () { quote ! { { struct _SchemarsDefaultSerialize < T > (T) ; impl serde :: Serialize for _SchemarsDefaultSerialize <# ty > { fn serialize < S > (& self , serializer : S) -> :: core :: result :: Result < S :: Ok , S :: Error > where S : serde :: Serializer { # ser_with (& self . 0 , serializer) } } # default_expr . map (| d | _SchemarsDefaultSerialize (d)) } } } else { default_expr }) }
};
}
