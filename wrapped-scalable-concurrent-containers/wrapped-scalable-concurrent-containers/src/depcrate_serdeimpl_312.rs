// Generated macro for impl_312 (impl)
macro_rules! Depcrate_serdeimpl_312 {
() => {
// Module: crate::serde
// Provides: {"impl_312"}
// Dependencies: {}
impl < 'd , K , V > Deserialize < 'd > for TreeIndex < K , V > where K : 'static + Clone + Deserialize < 'd > + Ord , V : 'static + Clone + Deserialize < 'd > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'd > , { deserializer . deserialize_map (TreeIndexVisitor :: < K , V > :: new ()) } }
};
}
