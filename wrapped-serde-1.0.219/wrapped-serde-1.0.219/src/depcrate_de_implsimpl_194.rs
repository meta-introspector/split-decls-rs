// Generated macro for impl_194 (impl)
macro_rules! Depcrate_de_implsimpl_194 {
() => {
// Module: crate::de::impls
// Provides: {"impl_194"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'de > Visitor < 'de > for StringVisitor { type Value = String ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a string") } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : Error , { Ok (v . to_owned ()) } fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : Error , { Ok (v) } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : Error , { match str :: from_utf8 (v) { Ok (s) => Ok (s . to_owned ()) , Err (_) => Err (Error :: invalid_value (Unexpected :: Bytes (v) , & self)) , } } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : Error , { match String :: from_utf8 (v) { Ok (s) => Ok (s) , Err (e) => Err (Error :: invalid_value (Unexpected :: Bytes (& e . into_bytes ()) , & self ,)) , } } }
};
}
