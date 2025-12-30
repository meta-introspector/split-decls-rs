// Generated macro for impl_74 (impl)
macro_rules! Depcrate_serializeimpl_74 {
() => {
// Module: crate::serialize
// Provides: {"impl_74"}
// Dependencies: {}
impl < S : Encoder , T > Encodable < S > for BTreeSet < T > where T : Encodable < S > + PartialEq + Ord , { fn encode (& self , s : & mut S) { s . emit_usize (self . len ()) ; for e in self { e . encode (s) ; } } }
};
}
