// Generated macro for impl_396 (impl)
macro_rules! Depcrate_revisionimpl_396 {
() => {
// Module: crate::revision
// Provides: {"impl_396"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for Revision { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) . map (| generation | Self { generation }) } }
};
}
