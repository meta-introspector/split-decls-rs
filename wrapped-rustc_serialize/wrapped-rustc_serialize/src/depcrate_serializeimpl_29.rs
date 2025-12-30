// Generated macro for impl_29 (impl)
macro_rules! Depcrate_serializeimpl_29 {
() => {
// Module: crate::serialize
// Provides: {"impl_29"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for str { fn encode (& self , s : & mut S) { s . emit_str (self) ; } }
};
}
