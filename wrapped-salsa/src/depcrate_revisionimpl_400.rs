// Generated macro for impl_400 (impl)
macro_rules! Depcrate_revisionimpl_400 {
() => {
// Module: crate::revision
// Provides: {"impl_400"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for AtomicRevision { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) . map (| data | Self { data : AtomicUsize :: new (data) , }) } }
};
}
