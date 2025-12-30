// Generated macro for impl_251 (impl)
macro_rules! Depcrate_de_implsimpl_251 {
() => {
// Module: crate::de::impls
// Provides: {"impl_251"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'de > Visitor < 'de > for PathBufVisitor { type Value = PathBuf ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("path string") } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : Error , { Ok (From :: from (v)) } fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : Error , { Ok (From :: from (v)) } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : Error , { str :: from_utf8 (v) . map (From :: from) . map_err (| _ | Error :: invalid_value (Unexpected :: Bytes (v) , & self)) } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : Error , { String :: from_utf8 (v) . map (From :: from) . map_err (| e | Error :: invalid_value (Unexpected :: Bytes (& e . into_bytes ()) , & self)) } }
};
}
