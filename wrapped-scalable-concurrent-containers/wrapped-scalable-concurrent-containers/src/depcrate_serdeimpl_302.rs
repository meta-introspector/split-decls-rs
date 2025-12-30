// Generated macro for impl_302 (impl)
macro_rules! Depcrate_serdeimpl_302 {
() => {
// Module: crate::serde
// Provides: {"impl_302"}
// Dependencies: {}
impl < 'd , K , V , H > Deserialize < 'd > for HashIndex < K , V , H > where K : Deserialize < 'd > + Eq + Hash , V : Deserialize < 'd > , H : BuildHasher + Default , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'd > , { deserializer . deserialize_map (HashIndexVisitor :: < K , V , H > :: new ()) } }
};
}
