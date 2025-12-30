// Generated macro for impl_48 (impl)
macro_rules! Depcrate_serializeimpl_48 {
() => {
// Module: crate::serialize
// Provides: {"impl_48"}
// Dependencies: {}
impl < S : Encoder , T : Encodable < S > > Encodable < S > for Option < T > { fn encode (& self , s : & mut S) { match * self { None => s . emit_u8 (0) , Some (ref v) => { s . emit_u8 (1) ; v . encode (s) ; } } } }
};
}
