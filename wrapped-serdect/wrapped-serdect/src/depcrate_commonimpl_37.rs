// Generated macro for impl_37 (impl)
macro_rules! Depcrate_commonimpl_37 {
() => {
// Module: crate::common
// Provides: {"impl_37"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Visitor < '_ > for StrIntoVecVisitor { type Value = Vec < u8 > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "a string") } fn visit_str < E > (self , v : & str) -> Result < Self :: Value , E > where E : Error , { base16ct :: mixed :: decode_vec (v) . map_err (E :: custom) } }
};
}
