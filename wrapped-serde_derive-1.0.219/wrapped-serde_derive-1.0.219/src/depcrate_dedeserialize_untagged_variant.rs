// Generated macro for deserialize_untagged_variant (function)
macro_rules! Depcrate_dedeserialize_untagged_variant {
() => {
// Module: crate::de
// Provides: {"deserialize_untagged_variant"}
// Dependencies: {}
fn deserialize_untagged_variant (params : & Parameters , variant : & Variant , cattrs : & attr :: Container , deserializer : TokenStream ,) -> Fragment { if let Some (path) = variant . attrs . deserialize_with () { let unwrap_fn = unwrap_to_variant_closure (params , variant , false) ; return quote_block ! { _serde :: __private :: Result :: map (# path (# deserializer) , # unwrap_fn) } ; } let variant_ident = & variant . ident ; match effective_style (variant) { Style :: Unit => { let this_value = & params . this_value ; let type_name = params . type_name () ; let variant_name = variant . ident . to_string () ; let default = variant . fields . first () . map (| field | { let default = Expr (expr_is_missing (field , cattrs)) ; quote ! ((# default)) }) ; quote_expr ! { match _serde :: Deserializer :: deserialize_any (# deserializer , _serde :: __private :: de :: UntaggedUnitVisitor :: new (# type_name , # variant_name)) { _serde :: __private :: Ok (()) => _serde :: __private :: Ok (# this_value ::# variant_ident # default) , _serde :: __private :: Err (__err) => _serde :: __private :: Err (__err) , } } } Style :: Newtype => deserialize_untagged_newtype_variant (variant_ident , params , & variant . fields [0] , & deserializer ,) , Style :: Tuple => deserialize_tuple (params , & variant . fields , cattrs , TupleForm :: Untagged (variant_ident , deserializer) ,) , Style :: Struct => deserialize_struct (params , & variant . fields , cattrs , StructForm :: Untagged (variant_ident , deserializer) ,) , } }
};
}
