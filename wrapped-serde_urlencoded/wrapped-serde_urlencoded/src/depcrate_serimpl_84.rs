// Generated macro for impl_84 (impl)
macro_rules! Depcrate_serimpl_84 {
() => {
// Module: crate::ser
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'input , 'output , Target : 'output + UrlEncodedTarget > Serializer < 'input , 'output , Target > { # [doc = " Returns a new `Serializer`."] pub fn new (urlencoder : & 'output mut UrlEncodedSerializer < 'input , Target > ,) -> Self { Serializer { urlencoder } } }
};
}
