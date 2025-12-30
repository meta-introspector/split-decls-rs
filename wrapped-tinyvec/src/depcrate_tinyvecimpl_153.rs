// Generated macro for impl_153 (impl)
macro_rules! Depcrate_tinyvecimpl_153 {
() => {
// Module: crate::tinyvec
// Provides: {"impl_153"}
// Dependencies: {}
# [cfg (feature = "serde")] # [cfg_attr (docs_rs , doc (cfg (feature = "serde")))] impl < 'de , A : Array > Deserialize < 'de > for TinyVec < A > where A :: Item : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_seq (TinyVecVisitor (PhantomData)) } }
};
}
