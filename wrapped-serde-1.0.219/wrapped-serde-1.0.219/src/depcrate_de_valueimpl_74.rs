// Generated macro for impl_74 (impl)
macro_rules! Depcrate_de_valueimpl_74 {
() => {
// Module: crate::de::value
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'de , E > de :: EnumAccess < 'de > for BorrowedStrDeserializer < 'de , E > where E : de :: Error , { type Error = E ; type Variant = private :: UnitOnly < E > ; fn variant_seed < T > (self , seed : T) -> Result < (T :: Value , Self :: Variant) , Self :: Error > where T : de :: DeserializeSeed < 'de > , { seed . deserialize (self) . map (private :: unit_only) } }
};
}
