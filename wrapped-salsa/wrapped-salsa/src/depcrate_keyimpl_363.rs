// Generated macro for impl_363 (impl)
macro_rules! Depcrate_keyimpl_363 {
() => {
// Module: crate::key
// Provides: {"impl_363"}
// Dependencies: {}
# [cfg (feature = "persistence")] impl < 'de > serde :: Deserialize < 'de > for DatabaseKeyIndex { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : serde :: Deserializer < 'de > , { let (key_index , ingredient_index) = serde :: Deserialize :: deserialize (deserializer) ? ; Ok (DatabaseKeyIndex { key_index , ingredient_index , }) } }
};
}
