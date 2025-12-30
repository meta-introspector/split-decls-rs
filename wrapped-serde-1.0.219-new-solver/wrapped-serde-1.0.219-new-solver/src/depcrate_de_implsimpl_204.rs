// Generated macro for impl_204 (impl)
macro_rules! Depcrate_de_implsimpl_204 {
() => {
// Module: crate::de::impls
// Provides: {"impl_204"}
// Dependencies: {}
# [cfg (any (feature = "std" , all (not (no_core_cstr) , feature = "alloc")))] impl < 'de > Visitor < 'de > for CStringVisitor { type Value = CString ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("byte array") } fn visit_seq < A > (self , mut seq : A) -> Result < Self :: Value , A :: Error > where A : SeqAccess < 'de > , { let capacity = size_hint :: cautious :: < u8 > (seq . size_hint ()) ; let mut values = Vec :: < u8 > :: with_capacity (capacity) ; while let Some (value) = tri ! (seq . next_element ()) { values . push (value) ; } CString :: new (values) . map_err (Error :: custom) } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : Error , { CString :: new (v) . map_err (Error :: custom) } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : Error , { CString :: new (v) . map_err (Error :: custom) } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : Error , { CString :: new (v) . map_err (Error :: custom) } fn visit_string < E > (self , v : String) -> Result < Self :: Value , E > where E : Error , { CString :: new (v) . map_err (Error :: custom) } }
};
}
