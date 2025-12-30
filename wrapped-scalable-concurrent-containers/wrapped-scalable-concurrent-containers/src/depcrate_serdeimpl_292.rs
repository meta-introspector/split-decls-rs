// Generated macro for impl_292 (impl)
macro_rules! Depcrate_serdeimpl_292 {
() => {
// Module: crate::serde
// Provides: {"impl_292"}
// Dependencies: {}
impl < 'd , K , V , H > Deserialize < 'd > for HashMap < K , V , H > where K : Deserialize < 'd > + Eq + Hash , V : Deserialize < 'd > , H : BuildHasher + Default , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'd > , { deserializer . deserialize_map (HashMapVisitor :: < K , V , H > :: new ()) } }
};
}
