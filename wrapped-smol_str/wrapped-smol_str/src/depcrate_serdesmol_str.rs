// Generated macro for smol_str (function)
macro_rules! Depcrate_serdesmol_str {
() => {
// Module: crate::serde
// Provides: {"smol_str"}
// Dependencies: {}
fn smol_str < 'de : 'a , 'a , D > (deserializer : D) -> Result < SmolStr , D :: Error > where D : Deserializer < 'de > , { struct SmolStrVisitor ; impl < 'a > Visitor < 'a > for SmolStrVisitor { type Value = SmolStr ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a string") } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : Error , { Ok (SmolStr :: from (v)) } fn visit_borrowed_str < E > (self , v : & 'a str) -> Result < Self :: Value , E > where E : Error , { Ok (SmolStr :: from (v)) } fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : Error , { Ok (SmolStr :: from (v)) } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : Error , { match core :: str :: from_utf8 (v) { Ok (s) => Ok (SmolStr :: from (s)) , Err (_) => Err (Error :: invalid_value (Unexpected :: Bytes (v) , & self)) , } } fn visit_borrowed_bytes < E > (self , v : & 'a [u8]) -> Result < Self :: Value , E > where E : Error , { match core :: str :: from_utf8 (v) { Ok (s) => Ok (SmolStr :: from (s)) , Err (_) => Err (Error :: invalid_value (Unexpected :: Bytes (v) , & self)) , } } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : Error , { match String :: from_utf8 (v) { Ok (s) => Ok (SmolStr :: from (s)) , Err (e) => Err (Error :: invalid_value (Unexpected :: Bytes (& e . into_bytes ()) , & self)) , } } } deserializer . deserialize_str (SmolStrVisitor) }
};
}
