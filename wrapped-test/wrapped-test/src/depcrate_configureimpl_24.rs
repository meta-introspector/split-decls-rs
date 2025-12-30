// Generated macro for impl_24 (impl)
macro_rules! Depcrate_configureimpl_24 {
() => {
// Module: crate::configure
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'de , T > DeserializeSeed < 'de > for Readable < T > where T : DeserializeSeed < 'de > , { type Value = T :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { self . 0 . deserialize (Readable (deserializer)) } }
};
}
