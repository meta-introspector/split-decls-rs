// Generated macro for impl_78 (impl)
macro_rules! Depcrate_serializeimpl_78 {
() => {
// Module: crate::serialize
// Provides: {"impl_78"}
// Dependencies: {}
impl < E : Encoder , T , S > Encodable < E > for HashSet < T , S > where T : Encodable < E > + Eq , S : BuildHasher , { fn encode (& self , s : & mut E) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
};
}
