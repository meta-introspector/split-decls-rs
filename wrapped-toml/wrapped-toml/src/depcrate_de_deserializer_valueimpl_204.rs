// Generated macro for impl_204 (impl)
macro_rules! Depcrate_de_deserializer_valueimpl_204 {
() => {
// Module: crate::de::deserializer::value
// Provides: {"impl_204"}
// Dependencies: {}
impl < 'de > serde_core :: de :: IntoDeserializer < 'de , Error > for Spanned < DeValue < 'de > > { type Deserializer = ValueDeserializer < 'de > ; fn into_deserializer (self) -> Self :: Deserializer { ValueDeserializer :: from (self) } }
};
}
