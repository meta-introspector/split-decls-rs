// Generated macro for impl_91 (impl)
macro_rules! Depcrate_cycleimpl_91 {
() => {
// Module: crate::cycle
// Provides: {"impl_91"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for AtomicIterationCount { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { IterationCount :: deserialize (deserializer) . map (Into :: into) } }
};
}
