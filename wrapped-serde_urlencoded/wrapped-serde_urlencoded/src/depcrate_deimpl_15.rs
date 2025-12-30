// Generated macro for impl_15 (impl)
macro_rules! Depcrate_deimpl_15 {
() => {
// Module: crate::de
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'de > Deserializer < 'de > { # [doc = " Returns a new `Deserializer`."] pub fn new (parser : UrlEncodedParse < 'de >) -> Self { Deserializer { inner : MapDeserializer :: new (PartIterator (parser)) , } } }
};
}
