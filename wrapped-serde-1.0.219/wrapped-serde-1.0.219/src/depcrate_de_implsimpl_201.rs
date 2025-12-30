// Generated macro for impl_201 (impl)
macro_rules! Depcrate_de_implsimpl_201 {
() => {
// Module: crate::de::impls
// Provides: {"impl_201"}
// Dependencies: {}
impl < 'a > Visitor < 'a > for BytesVisitor { type Value = & 'a [u8] ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a borrowed byte array") } fn visit_borrowed_bytes < E > (self , v : & 'a [u8]) -> Result < Self :: Value , E > where E : Error , { Ok (v) } fn visit_borrowed_str < E > (self , v : & 'a str) -> Result < Self :: Value , E > where E : Error , { Ok (v . as_bytes ()) } }
};
}
