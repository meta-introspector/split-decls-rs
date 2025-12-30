// Generated macro for deserialize_untagged_enum_after (function)
macro_rules! Depcrate_dedeserialize_untagged_enum_after {
() => {
// Module: crate::de
// Provides: {"deserialize_untagged_enum_after"}
// Dependencies: {}
fn deserialize_untagged_enum_after (params : & Parameters , variants : & [Variant] , cattrs : & attr :: Container , first_attempt : Option < Expr > ,) -> Fragment { let attempts = variants . iter () . filter (| variant | ! variant . attrs . skip_deserializing ()) . map (| variant | { Expr (deserialize_untagged_variant (params , variant , cattrs , quote ! (__deserializer) ,)) }) ; let fallthrough_msg = format ! ("data did not match any variant of untagged enum {}" , params . type_name ()) ; let fallthrough_msg = cattrs . expecting () . unwrap_or (& fallthrough_msg) ; let first_attempt = first_attempt . map (| expr | { quote ! { if let _serde :: __private :: Result ::< _ , __D :: Error >:: Ok (__ok) = (|| # expr) () { return _serde :: __private :: Ok (__ok) ; } } }) ; quote_block ! { let __content = < _serde :: __private :: de :: Content as _serde :: Deserialize >:: deserialize (__deserializer) ?; let __deserializer = _serde :: __private :: de :: ContentRefDeserializer ::< __D :: Error >:: new (& __content) ; # first_attempt # (if let _serde :: __private :: Ok (__ok) = # attempts { return _serde :: __private :: Ok (__ok) ; }) * _serde :: __private :: Err (_serde :: de :: Error :: custom (# fallthrough_msg)) } }
};
}
