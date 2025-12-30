// Generated macro for impl_82 (impl)
macro_rules! Depcrate_serializeimpl_82 {
() => {
// Module: crate::serialize
// Provides: {"impl_82"}
// Dependencies: {}
impl < E : Encoder , T , S > Encodable < E > for indexmap :: IndexSet < T , S > where T : Encodable < E > + Hash + Eq , S : BuildHasher , { fn encode (& self , s : & mut E) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
};
}
