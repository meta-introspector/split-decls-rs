// Generated macro for impl_154 (impl)
macro_rules! Depcrate_de_ignored_anyimpl_154 {
() => {
// Module: crate::de::ignored_any
// Provides: {"impl_154"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for IgnoredAny { # [inline] fn deserialize < D > (deserializer : D) -> Result < IgnoredAny , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_ignored_any (IgnoredAny) } }
};
}
