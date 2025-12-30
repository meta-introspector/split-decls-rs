// Generated macro for impl_237 (impl)
macro_rules! Depcrate_cldr_serde_numbersimpl_237 {
() => {
// Module: crate::cldr_serde::numbers
// Provides: {"impl_237"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for NumberingSystemData { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_map (NumberingSystemDataVisitor) } }
};
}
