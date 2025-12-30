// Generated macro for impl_195 (impl)
macro_rules! Depcrate_de_implsimpl_195 {
() => {
// Module: crate::de::impls
// Provides: {"impl_195"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'a , 'de > Visitor < 'de > for StringInPlaceVisitor < 'a > { type Value = () ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a string") } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : Error , { self . 0 . clear () ; self . 0 . push_str (v) ; Ok (()) } fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : Error , { * self . 0 = v ; Ok (()) } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : Error , { match str :: from_utf8 (v) { Ok (s) => { self . 0 . clear () ; self . 0 . push_str (s) ; Ok (()) } Err (_) => Err (Error :: invalid_value (Unexpected :: Bytes (v) , & self)) , } } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : Error , { match String :: from_utf8 (v) { Ok (s) => { * self . 0 = s ; Ok (()) } Err (e) => Err (Error :: invalid_value (Unexpected :: Bytes (& e . into_bytes ()) , & self ,)) , } } }
};
}
