// Generated macro for impl_24 (impl)
macro_rules! Depcrate_arrayvecimpl_24 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_24"}
// Dependencies: {}
# [cfg (feature = "serde")] # [cfg_attr (docs_rs , doc (cfg (feature = "serde")))] impl < 'de , A : Array > Deserialize < 'de > for ArrayVec < A > where A :: Item : Deserialize < 'de > , { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_seq (ArrayVecVisitor (PhantomData)) } }
};
}
