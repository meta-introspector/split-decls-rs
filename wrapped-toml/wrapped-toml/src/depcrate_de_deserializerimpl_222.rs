// Generated macro for impl_222 (impl)
macro_rules! Depcrate_de_deserializerimpl_222 {
() => {
// Module: crate::de::deserializer
// Provides: {"impl_222"}
// Dependencies: {}
impl < 'de > serde_core :: de :: IntoDeserializer < 'de , Error > for Spanned < DeTable < 'de > > { type Deserializer = Deserializer < 'de > ; fn into_deserializer (self) -> Self :: Deserializer { Deserializer :: from (self) } }
};
}
