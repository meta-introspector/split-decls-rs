// Generated macro for deserialize_externally_tagged_newtype_variant (function)
macro_rules! Depcrate_dedeserialize_externally_tagged_newtype_variant {
() => {
// Module: crate::de
// Provides: {"deserialize_externally_tagged_newtype_variant"}
// Dependencies: {}
fn deserialize_externally_tagged_newtype_variant (variant_ident : & syn :: Ident , params : & Parameters , field : & Field , cattrs : & attr :: Container ,) -> Fragment { let this_value = & params . this_value ; if field . attrs . skip_deserializing () { let default = Expr (expr_is_missing (field , cattrs)) ; return quote_block ! { _serde :: de :: VariantAccess :: unit_variant (__variant) ?; _serde :: __private :: Ok (# this_value ::# variant_ident (# default)) } ; } match field . attrs . deserialize_with () { None => { let field_ty = field . ty ; let span = field . original . span () ; let func = quote_spanned ! (span => _serde :: de :: VariantAccess :: newtype_variant ::<# field_ty >) ; quote_expr ! { _serde :: __private :: Result :: map (# func (__variant) , # this_value ::# variant_ident) } } Some (path) => { let (wrapper , wrapper_ty) = wrap_deserialize_field_with (params , field . ty , path) ; quote_block ! { # wrapper _serde :: __private :: Result :: map (_serde :: de :: VariantAccess :: newtype_variant ::<# wrapper_ty > (__variant) , | __wrapper | # this_value ::# variant_ident (__wrapper . value)) } } } }
};
}
