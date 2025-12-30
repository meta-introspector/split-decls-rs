// Generated macro for impl_212 (impl)
macro_rules! Depcrate_serdeimpl_212 {
() => {
// Module: crate::serde
// Provides: {"impl_212"}
// Dependencies: {}
impl Serialize for & ByteStr { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let bytes = self . as_bytes () ; if serializer . is_human_readable () { match core :: str :: from_utf8 (bytes) { Ok (s) => serializer . serialize_str (s) , Err (_) => serializer . serialize_bytes (bytes) , } } else { serializer . serialize_bytes (bytes) } } }
};
}
