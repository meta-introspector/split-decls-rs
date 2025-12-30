// Generated macro for impl_59 (impl)
macro_rules! Depcrate_de_valueimpl_59 {
() => {
// Module: crate::de::value
// Provides: {"impl_59"}
// Dependencies: {}
impl < 'de , E > de :: EnumAccess < 'de > for U32Deserializer < E > where E : de :: Error , { type Error = E ; type Variant = private :: UnitOnly < E > ; fn variant_seed < T > (self , seed : T) -> Result < (T :: Value , Self :: Variant) , Self :: Error > where T : de :: DeserializeSeed < 'de > , { seed . deserialize (self) . map (private :: unit_only) } }
};
}
