// Generated macro for impl_221 (impl)
macro_rules! Depcrate_unique_arcimpl_221 {
() => {
// Module: crate::unique_arc
// Provides: {"impl_221"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , T : Deserialize < 'de > > Deserialize < 'de > for UniqueArc < T > { fn deserialize < D > (deserializer : D) -> Result < UniqueArc < T > , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { T :: deserialize (deserializer) . map (UniqueArc :: new) } }
};
}
