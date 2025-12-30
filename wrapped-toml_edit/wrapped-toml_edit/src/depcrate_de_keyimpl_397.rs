// Generated macro for impl_397 (impl)
macro_rules! Depcrate_de_keyimpl_397 {
() => {
// Module: crate::de::key
// Provides: {"impl_397"}
// Dependencies: {}
impl < 'de > serde_core :: de :: EnumAccess < 'de > for KeyDeserializer { type Error = Error ; type Variant = UnitOnly < < Self as serde_core :: de :: EnumAccess < 'de > > :: Error > ; fn variant_seed < T > (self , seed : T) -> Result < (< T as serde_core :: de :: DeserializeSeed < 'de > > :: Value , < Self as serde_core :: de :: EnumAccess < 'de > > :: Variant) , < Self as serde_core :: de :: EnumAccess < 'de > > :: Error > where T : serde_core :: de :: DeserializeSeed < 'de > , { seed . deserialize (self) . map (unit_only) } }
};
}
