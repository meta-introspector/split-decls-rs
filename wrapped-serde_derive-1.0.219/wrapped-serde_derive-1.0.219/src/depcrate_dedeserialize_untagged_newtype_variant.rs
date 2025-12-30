// Generated macro for deserialize_untagged_newtype_variant (function)
macro_rules! Depcrate_dedeserialize_untagged_newtype_variant {
() => {
// Module: crate::de
// Provides: {"deserialize_untagged_newtype_variant"}
// Dependencies: {}
fn deserialize_untagged_newtype_variant (variant_ident : & syn :: Ident , params : & Parameters , field : & Field , deserializer : & TokenStream ,) -> Fragment { let this_value = & params . this_value ; let field_ty = field . ty ; match field . attrs . deserialize_with () { None => { let span = field . original . span () ; let func = quote_spanned ! (span => <# field_ty as _serde :: Deserialize >:: deserialize) ; quote_expr ! { _serde :: __private :: Result :: map (# func (# deserializer) , # this_value ::# variant_ident) } } Some (path) => { quote_block ! { let __value : _serde :: __private :: Result <# field_ty , _ > = # path (# deserializer) ; _serde :: __private :: Result :: map (__value , # this_value ::# variant_ident) } } } }
};
}
