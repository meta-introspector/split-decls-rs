// Generated macro for impl_529 (impl)
macro_rules! Depcrate_ser_value_mapimpl_529 {
() => {
// Module: crate::ser::value::map
// Provides: {"impl_529"}
// Dependencies: {}
impl < 'd > SerializeMap < 'd > { pub (crate) fn map (dst : & 'd mut String , style : Style) -> Result < Self , Error > { Ok (Self :: Table (SerializeTable :: map (dst , style) ?)) } pub (crate) fn struct_ (name : & 'static str , dst : & 'd mut String , style : Style ,) -> Result < Self , Error > { if toml_datetime :: ser :: is_datetime (name) { Ok (Self :: Datetime (SerializeDatetime :: new (dst))) } else { Ok (Self :: map (dst , style) ?) } } }
};
}
