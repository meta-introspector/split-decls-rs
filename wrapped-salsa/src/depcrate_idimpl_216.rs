// Generated macro for impl_216 (impl)
macro_rules! Depcrate_idimpl_216 {
() => {
// Module: crate::id
// Provides: {"impl_216"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for Id { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { serde :: Deserialize :: deserialize (deserializer) . map (Self :: from_bits) } }
};
}
