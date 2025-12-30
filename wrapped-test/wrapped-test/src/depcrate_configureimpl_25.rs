// Generated macro for impl_25 (impl)
macro_rules! Depcrate_configureimpl_25 {
() => {
// Module: crate::configure
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'de , T > DeserializeSeed < 'de > for Compact < T > where T : DeserializeSeed < 'de > , { type Value = T :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { self . 0 . deserialize (Compact (deserializer)) } }
};
}
