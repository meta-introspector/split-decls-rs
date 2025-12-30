// Generated macro for impl_110 (impl)
macro_rules! Depcrate_internallyimpl_110 {
() => {
// Module: crate::internally
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'de , K > DeserializeSeed < 'de > for StringKeySeed < K > where K : DeserializeSeed < 'de > , { type Value = K :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { self . seed . deserialize (StringKeyDeserializer { delegate : deserializer , }) } }
};
}
