// Generated macro for impl_160 (impl)
macro_rules! Depcrate_de_deserializer_keyimpl_160 {
() => {
// Module: crate::de::deserializer::key
// Provides: {"impl_160"}
// Dependencies: {}
impl < 'de > serde_core :: de :: EnumAccess < 'de > for KeyDeserializer < 'de > { type Error = Error ; type Variant = UnitOnly < Self :: Error > ; fn variant_seed < T > (self , seed : T) -> Result < (T :: Value , Self :: Variant) , Self :: Error > where T : serde_core :: de :: DeserializeSeed < 'de > , { seed . deserialize (self) . map (unit_only) } }
};
}
