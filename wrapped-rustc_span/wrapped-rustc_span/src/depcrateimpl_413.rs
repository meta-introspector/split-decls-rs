// Generated macro for impl_413 (impl)
macro_rules! Depcrateimpl_413 {
() => {
// Module: crate
// Provides: {"impl_413"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for RelativeBytePos { fn encode (& self , s : & mut S) { s . emit_u32 (self . 0) ; } }
};
}
