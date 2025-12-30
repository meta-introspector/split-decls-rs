// Generated macro for impl_162 (impl)
macro_rules! Depcrate_de_implsimpl_162 {
() => {
// Module: crate::de::impls
// Provides: {"impl_162"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for () { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_unit (UnitVisitor) } }
};
}
