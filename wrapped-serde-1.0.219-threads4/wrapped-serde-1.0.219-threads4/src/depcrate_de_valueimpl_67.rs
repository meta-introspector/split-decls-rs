// Generated macro for impl_67 (impl)
macro_rules! Depcrate_de_valueimpl_67 {
() => {
// Module: crate::de::value
// Provides: {"impl_67"}
// Dependencies: {}
impl < 'de , 'a , E > de :: EnumAccess < 'de > for StrDeserializer < 'a , E > where E : de :: Error , { type Error = E ; type Variant = private :: UnitOnly < E > ; fn variant_seed < T > (self , seed : T) -> Result < (T :: Value , Self :: Variant) , Self :: Error > where T : de :: DeserializeSeed < 'de > , { seed . deserialize (self) . map (private :: unit_only) } }
};
}
