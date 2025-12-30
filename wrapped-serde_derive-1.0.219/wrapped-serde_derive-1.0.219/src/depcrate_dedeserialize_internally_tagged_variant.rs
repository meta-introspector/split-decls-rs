// Generated macro for deserialize_internally_tagged_variant (function)
macro_rules! Depcrate_dedeserialize_internally_tagged_variant {
() => {
// Module: crate::de
// Provides: {"deserialize_internally_tagged_variant"}
// Dependencies: {}
fn deserialize_internally_tagged_variant (params : & Parameters , variant : & Variant , cattrs : & attr :: Container , deserializer : TokenStream ,) -> Fragment { if variant . attrs . deserialize_with () . is_some () { return deserialize_untagged_variant (params , variant , cattrs , deserializer) ; } let variant_ident = & variant . ident ; match effective_style (variant) { Style :: Unit => { let this_value = & params . this_value ; let type_name = params . type_name () ; let variant_name = variant . ident . to_string () ; let default = variant . fields . first () . map (| field | { let default = Expr (expr_is_missing (field , cattrs)) ; quote ! ((# default)) }) ; quote_block ! { _serde :: Deserializer :: deserialize_any (# deserializer , _serde :: __private :: de :: InternallyTaggedUnitVisitor :: new (# type_name , # variant_name)) ?; _serde :: __private :: Ok (# this_value ::# variant_ident # default) } } Style :: Newtype => deserialize_untagged_newtype_variant (variant_ident , params , & variant . fields [0] , & deserializer ,) , Style :: Struct => deserialize_struct (params , & variant . fields , cattrs , StructForm :: InternallyTagged (variant_ident , deserializer) ,) , Style :: Tuple => unreachable ! ("checked in serde_derive_internals") , } }
};
}
