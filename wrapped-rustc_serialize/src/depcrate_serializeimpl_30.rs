// Generated macro for impl_30 (impl)
macro_rules! Depcrate_serializeimpl_30 {
() => {
// Module: crate::serialize
// Provides: {"impl_30"}
// Dependencies: {}
impl < S : Encoder > Encodable < S > for String { fn encode (& self , s : & mut S) { s . emit_str (& self) ; } }
};
}
