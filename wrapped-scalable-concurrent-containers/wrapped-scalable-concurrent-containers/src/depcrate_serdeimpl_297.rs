// Generated macro for impl_297 (impl)
macro_rules! Depcrate_serdeimpl_297 {
() => {
// Module: crate::serde
// Provides: {"impl_297"}
// Dependencies: {}
impl < 'd , K , H > Deserialize < 'd > for HashSet < K , H > where K : Deserialize < 'd > + Eq + Hash , H : BuildHasher + Default , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'd > , { deserializer . deserialize_seq (HashSetVisitor :: < K , H > :: new ()) } }
};
}
