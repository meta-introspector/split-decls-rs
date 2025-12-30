// Generated macro for impl_482 (impl)
macro_rules! Depcrate_ser_mapimpl_482 {
() => {
// Module: crate::ser::map
// Provides: {"impl_482"}
// Dependencies: {}
impl SerializeMap { pub (crate) fn map (len : Option < usize >) -> Self { Self :: Table (SerializeInlineTable :: map (len)) } pub (crate) fn struct_ (name : & 'static str , len : Option < usize >) -> Self { if toml_datetime :: ser :: is_datetime (name) { Self :: Datetime (SerializeDatetime :: new ()) } else { Self :: map (len) } } }
};
}
