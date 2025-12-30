// Generated macro for impl_41 (impl)
macro_rules! Depcrate_commonimpl_41 {
() => {
// Module: crate::common
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl Visitor < '_ > for VecVisitor { type Value = Vec < u8 > ; fn expecting (& self , formatter : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (formatter , "a bytestring") } fn visit_bytes < E > (self , v : & [u8]) -> Result < Self :: Value , E > where E : Error , { Ok (v . into ()) } fn visit_byte_buf < E > (self , v : Vec < u8 >) -> Result < Self :: Value , E > where E : Error , { Ok (v) } }
};
}
