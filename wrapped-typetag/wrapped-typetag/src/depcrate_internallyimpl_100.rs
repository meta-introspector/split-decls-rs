// Generated macro for impl_100 (impl)
macro_rules! Depcrate_internallyimpl_100 {
() => {
// Module: crate::internally
// Provides: {"impl_100"}
// Dependencies: {}
impl < 'de > DeserializeSeed < 'de > for KeyVisitor { type Value = Key ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (self) } }
};
}
