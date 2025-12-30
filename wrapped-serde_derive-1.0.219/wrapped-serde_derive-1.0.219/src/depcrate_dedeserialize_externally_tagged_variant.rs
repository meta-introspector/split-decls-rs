// Generated macro for deserialize_externally_tagged_variant (function)
macro_rules! Depcrate_dedeserialize_externally_tagged_variant {
() => {
// Module: crate::de
// Provides: {"deserialize_externally_tagged_variant"}
// Dependencies: {}
fn deserialize_externally_tagged_variant (params : & Parameters , variant : & Variant , cattrs : & attr :: Container ,) -> Fragment { if let Some (path) = variant . attrs . deserialize_with () { let (wrapper , wrapper_ty , unwrap_fn) = wrap_deserialize_variant_with (params , variant , path) ; return quote_block ! { # wrapper _serde :: __private :: Result :: map (_serde :: de :: VariantAccess :: newtype_variant ::<# wrapper_ty > (__variant) , # unwrap_fn) } ; } let variant_ident = & variant . ident ; match variant . style { Style :: Unit => { let this_value = & params . this_value ; quote_block ! { _serde :: de :: VariantAccess :: unit_variant (__variant) ?; _serde :: __private :: Ok (# this_value ::# variant_ident) } } Style :: Newtype => deserialize_externally_tagged_newtype_variant (variant_ident , params , & variant . fields [0] , cattrs ,) , Style :: Tuple => deserialize_tuple (params , & variant . fields , cattrs , TupleForm :: ExternallyTagged (variant_ident) ,) , Style :: Struct => deserialize_struct (params , & variant . fields , cattrs , StructForm :: ExternallyTagged (variant_ident) ,) , } }
};
}
