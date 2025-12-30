// Generated macro for impl_429 (impl)
macro_rules! Depcrateimpl_429 {
() => {
// Module: crate
// Provides: {"impl_429"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for RelativeBytePos { fn encode (& self , s : & mut S) { s . emit_u32 (self . 0) ; } }
};
}
