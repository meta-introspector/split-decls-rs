// Generated macro for impl_410 (impl)
macro_rules! Depcrateimpl_410 {
() => {
// Module: crate
// Provides: {"impl_410"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for BytePos { fn encode (& self , s : & mut S) { s . emit_u32 (self . 0) ; } }
};
}
