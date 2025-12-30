// Generated macro for impl_95 (impl)
macro_rules! Depcrate_cycleimpl_95 {
() => {
// Module: crate::cycle
// Provides: {"impl_95"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for CycleHeads { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let vec : ThinVec < CycleHead > = serde :: Deserialize :: deserialize (deserializer) ? ; Ok (CycleHeads (vec)) } }
};
}
