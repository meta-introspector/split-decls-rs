// Generated macro for impl_175 (impl)
macro_rules! Depcrate_inheritimpl_175 {
() => {
// Module: crate::inherit
// Provides: {"impl_175"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for True { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_bool (True) } }
};
}
