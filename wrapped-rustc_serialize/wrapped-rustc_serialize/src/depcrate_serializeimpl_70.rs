// Generated macro for impl_70 (impl)
macro_rules! Depcrate_serializeimpl_70 {
() => {
// Module: crate::serialize
// Provides: {"impl_70"}
// Dependencies: {}
impl < S : Encoder , T : Encodable < S > > Encodable < S > for VecDeque < T > { fn encode (& self , s : & mut S) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
};
}
