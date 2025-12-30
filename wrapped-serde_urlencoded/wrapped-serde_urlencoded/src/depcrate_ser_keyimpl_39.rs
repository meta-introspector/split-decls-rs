// Generated macro for impl_39 (impl)
macro_rules! Depcrate_ser_keyimpl_39 {
() => {
// Module: crate::ser::key
// Provides: {"impl_39"}
// Dependencies: {}
impl < End , Ok > Sink for KeySink < End > where End : for < 'key > FnOnce (Key < 'key >) -> Result < Ok , Error > , { type Ok = Ok ; fn serialize_static_str (self , value : & 'static str) -> Result < Ok , Error > { (self . end) (Key :: Static (value)) } fn serialize_str (self , value : & str) -> Result < Ok , Error > { (self . end) (Key :: Dynamic (value . into ())) } fn serialize_string (self , value : String) -> Result < Ok , Error > { (self . end) (Key :: Dynamic (value . into ())) } fn serialize_none (self) -> Result < Ok , Error > { Err (self . unsupported ("none")) } fn serialize_some < T : ? Sized + Serialize > (self , _value : & T ,) -> Result < Ok , Error > { Err (self . unsupported ("some")) } fn unsupported (self , type_str : & 'static str) -> Error { Error :: Custom (format ! ("unsupported key type: {type_str}") . into ()) } }
};
}
