// Generated macro for ValueSink (struct)
macro_rules! Depcrate_ser_valueValueSink {
() => {
// Module: crate::ser::value
// Provides: {"ValueSink"}
// Dependencies: {}
pub struct ValueSink < 'input , 'key , 'target , Target > where Target : UrlEncodedTarget , { urlencoder : & 'target mut UrlEncodedSerializer < 'input , Target > , key : & 'key str , }
};
}
