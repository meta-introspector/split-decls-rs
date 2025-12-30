// Generated macro for deserialize_body (function)
macro_rules! Depcrate_dedeserialize_body {
() => {
// Module: crate::de
// Provides: {"deserialize_body"}
// Dependencies: {}
fn deserialize_body (cont : & Container , params : & Parameters) -> Fragment { if cont . attrs . transparent () { deserialize_transparent (cont , params) } else if let Some (type_from) = cont . attrs . type_from () { deserialize_from (type_from) } else if let Some (type_try_from) = cont . attrs . type_try_from () { deserialize_try_from (type_try_from) } else if let attr :: Identifier :: No = cont . attrs . identifier () { match & cont . data { Data :: Enum (variants) => deserialize_enum (params , variants , & cont . attrs) , Data :: Struct (Style :: Struct , fields) => { deserialize_struct (params , fields , & cont . attrs , StructForm :: Struct) } Data :: Struct (Style :: Tuple , fields) | Data :: Struct (Style :: Newtype , fields) => { deserialize_tuple (params , fields , & cont . attrs , TupleForm :: Tuple) } Data :: Struct (Style :: Unit , _) => deserialize_unit_struct (params , & cont . attrs) , } } else { match & cont . data { Data :: Enum (variants) => deserialize_custom_identifier (params , variants , & cont . attrs) , Data :: Struct (_ , _) => unreachable ! ("checked in serde_derive_internals") , } } }
};
}
