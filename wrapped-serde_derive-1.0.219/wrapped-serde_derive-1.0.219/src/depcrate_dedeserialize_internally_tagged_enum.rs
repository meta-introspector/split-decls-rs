// Generated macro for deserialize_internally_tagged_enum (function)
macro_rules! Depcrate_dedeserialize_internally_tagged_enum {
() => {
// Module: crate::de
// Provides: {"deserialize_internally_tagged_enum"}
// Dependencies: {}
fn deserialize_internally_tagged_enum (params : & Parameters , variants : & [Variant] , cattrs : & attr :: Container , tag : & str ,) -> Fragment { let (variants_stmt , variant_visitor) = prepare_enum_variant_enum (variants) ; let variant_arms = variants . iter () . enumerate () . filter (| & (_ , variant) | ! variant . attrs . skip_deserializing ()) . map (| (i , variant) | { let variant_name = field_i (i) ; let block = Match (deserialize_internally_tagged_variant (params , variant , cattrs , quote ! (__deserializer) ,)) ; quote ! { __Field ::# variant_name => # block } }) ; let expecting = format ! ("internally tagged enum {}" , params . type_name ()) ; let expecting = cattrs . expecting () . unwrap_or (& expecting) ; quote_block ! { # variant_visitor # variants_stmt let (__tag , __content) = _serde :: Deserializer :: deserialize_any (__deserializer , _serde :: __private :: de :: TaggedContentVisitor ::< __Field >:: new (# tag , # expecting)) ?; let __deserializer = _serde :: __private :: de :: ContentDeserializer ::< __D :: Error >:: new (__content) ; match __tag { # (# variant_arms) * } } }
};
}
