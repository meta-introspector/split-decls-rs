// Generated macro for impl_191 (impl)
macro_rules! Depcrate_de_implsimpl_191 {
() => {
// Module: crate::de::impls
// Provides: {"impl_191"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for char { # [inline] fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_char (CharVisitor) } }
};
}
