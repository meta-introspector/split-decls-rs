// Generated macro for impl_166 (impl)
macro_rules! Depcrate_de_implsimpl_166 {
() => {
// Module: crate::de::impls
// Provides: {"impl_166"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for bool { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_bool (BoolVisitor) } }
};
}
