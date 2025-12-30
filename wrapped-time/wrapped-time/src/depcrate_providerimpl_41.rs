// Generated macro for impl_41 (impl)
macro_rules! Depcrate_providerimpl_41 {
() => {
// Module: crate::provider
// Provides: {"impl_41"}
// Dependencies: {}
# [cfg (feature = "serde")] impl < 'de > serde :: Deserialize < 'de > for VariantOffsetsWithMetazoneMembershipKind { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { < _ > :: deserialize (deserializer) . map (Self :: from_unaligned) } }
};
}
