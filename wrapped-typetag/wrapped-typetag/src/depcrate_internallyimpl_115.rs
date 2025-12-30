// Generated macro for impl_115 (impl)
macro_rules! Depcrate_internallyimpl_115 {
() => {
// Module: crate::internally
// Provides: {"impl_115"}
// Dependencies: {}
impl < 'de > DeserializeSeed < 'de > for DefaultKey { type Value = () ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_str (self) } }
};
}
