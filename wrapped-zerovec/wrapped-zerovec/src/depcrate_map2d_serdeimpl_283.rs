// Generated macro for impl_283 (impl)
macro_rules! Depcrate_map2d_serdeimpl_283 {
() => {
// Module: crate::map2d::serde
// Provides: {"impl_283"}
// Dependencies: {}
impl < 'de , K1 , V > Deserialize < 'de > for TupleVecMap < K1 , V > where K1 : Deserialize < 'de > , V : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_map (TupleVecMapVisitor :: < K1 , V > :: new ()) } }
};
}
