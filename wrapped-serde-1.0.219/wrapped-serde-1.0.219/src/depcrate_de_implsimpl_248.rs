// Generated macro for impl_248 (impl)
macro_rules! Depcrate_de_implsimpl_248 {
() => {
// Module: crate::de::impls
// Provides: {"impl_248"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > Visitor < 'a > for PathVisitor { type Value = & 'a Path ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a borrowed path") } fn visit_borrowed_str < E > (self , v : & 'a str) -> Result < Self :: Value , E > where E : Error , { Ok (v . as_ref ()) } fn visit_borrowed_bytes < E > (self , v : & 'a [u8]) -> Result < Self :: Value , E > where E : Error , { str :: from_utf8 (v) . map (AsRef :: as_ref) . map_err (| _ | Error :: invalid_value (Unexpected :: Bytes (v) , & self)) } }
};
}
