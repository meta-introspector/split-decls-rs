// Generated macro for impl_405 (impl)
macro_rules! Depcrate_revisionimpl_405 {
() => {
// Module: crate::revision
// Provides: {"impl_405"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for OptionalAtomicRevision { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) . map (| data | Self { data : AtomicUsize :: new (data) , }) } }
};
}
