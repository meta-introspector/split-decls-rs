// Generated macro for impl_307 (impl)
macro_rules! Depcrate_serdeimpl_307 {
() => {
// Module: crate::serde
// Provides: {"impl_307"}
// Dependencies: {}
impl < 'd , K , V , H > Deserialize < 'd > for HashCache < K , V , H > where K : Deserialize < 'd > + Eq + Hash , V : Deserialize < 'd > , H : BuildHasher + Default , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'd > , { deserializer . deserialize_map (HashCacheVisitor :: < K , V , H > :: new ()) } }
};
}
