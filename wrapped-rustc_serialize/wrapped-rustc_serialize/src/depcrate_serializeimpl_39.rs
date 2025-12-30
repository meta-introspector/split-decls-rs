// Generated macro for impl_39 (impl)
macro_rules! Depcrate_serializeimpl_39 {
() => {
// Module: crate::serialize
// Provides: {"impl_39"}
// Dependencies: {}
impl < S : Encoder , T : Encodable < S > > Encodable < S > for [T] { default fn encode (& self , s : & mut S) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
};
}
