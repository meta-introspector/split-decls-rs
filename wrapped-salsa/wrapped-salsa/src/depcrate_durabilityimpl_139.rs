// Generated macro for impl_139 (impl)
macro_rules! Depcrate_durabilityimpl_139 {
() => {
// Module: crate::durability
// Provides: {"impl_139"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for Durability { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { u8 :: deserialize (deserializer) . map (| value | Self (DurabilityVal :: from (value))) } }
};
}
