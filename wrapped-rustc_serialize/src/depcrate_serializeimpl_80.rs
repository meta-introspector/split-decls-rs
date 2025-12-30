// Generated macro for impl_80 (impl)
macro_rules! Depcrate_serializeimpl_80 {
() => {
// Module: crate::serialize
// Provides: {"impl_80"}
// Dependencies: {}
impl < E : Encoder , K , V , S > Encodable < E > for indexmap :: IndexMap < K , V , S > where K : Encodable < E > + Hash + Eq , V : Encodable < E > , S : BuildHasher , { fn encode (& self , e : & mut E) { e . emit_usize (self . len ()) ; for (key , val) in self { key . encode (e) ; val . encode (e) ; } } }
};
}
