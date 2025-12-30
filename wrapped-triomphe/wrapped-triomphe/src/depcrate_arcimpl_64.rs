// Generated macro for impl_64 (impl)
macro_rules! Depcrate_arcimpl_64 {
() => {
// Module: crate::arc
// Provides: {"impl_64"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de , T : Deserialize < 'de > > Deserialize < 'de > for Arc < T > { fn deserialize < D > (deserializer : D) -> Result < Arc < T > , D :: Error > where D : :: serde :: de :: Deserializer < 'de > , { T :: deserialize (deserializer) . map (Arc :: new) } }
};
}
