// Generated macro for impl_426 (impl)
macro_rules! Depcrateimpl_426 {
() => {
// Module: crate
// Provides: {"impl_426"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for BytePos { fn encode (& self , s : & mut S) { s . emit_u32 (self . 0) ; } }
};
}
