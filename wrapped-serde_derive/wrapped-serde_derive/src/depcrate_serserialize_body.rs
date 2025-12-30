// Generated macro for serialize_body (function)
macro_rules! Depcrate_serserialize_body {
() => {
// Module: crate::ser
// Provides: {"serialize_body"}
// Dependencies: {}
fn serialize_body (cont : & Container , params : & Parameters) -> Fragment { if cont . attrs . transparent () { serialize_transparent (cont , params) } else if let Some (type_into) = cont . attrs . type_into () { serialize_into (params , type_into) } else { match & cont . data { Data :: Enum (variants) => serialize_enum (params , variants , & cont . attrs) , Data :: Struct (Style :: Struct , fields) => serialize_struct (params , fields , & cont . attrs) , Data :: Struct (Style :: Tuple , fields) => { serialize_tuple_struct (params , fields , & cont . attrs) } Data :: Struct (Style :: Newtype , fields) => { serialize_newtype_struct (params , & fields [0] , & cont . attrs) } Data :: Struct (Style :: Unit , _) => serialize_unit_struct (& cont . attrs) , } } }
};
}
