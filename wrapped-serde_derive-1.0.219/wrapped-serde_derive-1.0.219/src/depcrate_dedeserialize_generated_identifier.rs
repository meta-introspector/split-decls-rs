// Generated macro for deserialize_generated_identifier (function)
macro_rules! Depcrate_dedeserialize_generated_identifier {
() => {
// Module: crate::de
// Provides: {"deserialize_generated_identifier"}
// Dependencies: {}
fn deserialize_generated_identifier (deserialized_fields : & [FieldWithAliases] , has_flatten : bool , is_variant : bool , ignore_variant : Option < TokenStream > , fallthrough : Option < TokenStream > ,) -> Fragment { let this_value = quote ! (__Field) ; let field_idents : & Vec < _ > = & deserialized_fields . iter () . map (| field | & field . ident) . collect () ; let visitor_impl = Stmts (deserialize_identifier (& this_value , deserialized_fields , is_variant , fallthrough , None , ! is_variant && has_flatten , None ,)) ; let lifetime = if ! is_variant && has_flatten { Some (quote ! (<'de >)) } else { None } ; quote_block ! { # [allow (non_camel_case_types)] # [doc (hidden)] enum __Field # lifetime { # (# field_idents ,) * # ignore_variant } # [doc (hidden)] struct __FieldVisitor ; # [automatically_derived] impl <'de > _serde :: de :: Visitor <'de > for __FieldVisitor { type Value = __Field # lifetime ; # visitor_impl } # [automatically_derived] impl <'de > _serde :: Deserialize <'de > for __Field # lifetime { # [inline] fn deserialize < __D > (__deserializer : __D) -> _serde :: __private :: Result < Self , __D :: Error > where __D : _serde :: Deserializer <'de >, { _serde :: Deserializer :: deserialize_identifier (__deserializer , __FieldVisitor) } } } }
};
}
