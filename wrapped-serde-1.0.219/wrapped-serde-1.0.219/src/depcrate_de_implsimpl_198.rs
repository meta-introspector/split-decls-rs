// Generated macro for impl_198 (impl)
macro_rules! Depcrate_de_implsimpl_198 {
() => {
// Module: crate::de::impls
// Provides: {"impl_198"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for StrVisitor { type Value = & 'a str ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a borrowed string") } fn visit_borrowed_str < E > (self , v : & 'a str) -> Result < Self :: Value , E > where E : Error , { Ok (v) } fn visit_borrowed_bytes < E > (self , v : & 'a [u8]) -> Result < Self :: Value , E > where E : Error , { str :: from_utf8 (v) . map_err (| _ | Error :: invalid_value (Unexpected :: Bytes (v) , & self)) } }
};
}
