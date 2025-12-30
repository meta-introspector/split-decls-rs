// Generated macro for impl_72 (impl)
macro_rules! Depcrate_serializeimpl_72 {
() => {
// Module: crate::serialize
// Provides: {"impl_72"}
// Dependencies: {}
impl < S : Encoder , K , V > Encodable < S > for BTreeMap < K , V > where K : Encodable < S > + PartialEq + Ord , V : Encodable < S > , { fn encode (& self , e : & mut S) { e . emit_usize (self . len ()) ; for (key , val) in self { key . encode (e) ; val . encode (e) ; } } }
};
}
