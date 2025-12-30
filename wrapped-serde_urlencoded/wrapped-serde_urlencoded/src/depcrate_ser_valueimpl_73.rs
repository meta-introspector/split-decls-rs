// Generated macro for impl_73 (impl)
macro_rules! Depcrate_ser_valueimpl_73 {
() => {
// Module: crate::ser::value
// Provides: {"impl_73"}
// Dependencies: {}
impl < 'input , 'key , 'target , Target > ValueSink < 'input , 'key , 'target , Target > where Target : 'target + UrlEncodedTarget , { pub fn new (urlencoder : & 'target mut UrlEncodedSerializer < 'input , Target > , key : & 'key str ,) -> Self { ValueSink { urlencoder , key } } }
};
}
