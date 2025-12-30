// Generated macro for impl_82 (impl)
macro_rules! Depcrate_de_valueimpl_82 {
() => {
// Module: crate::de::value
// Provides: {"impl_82"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'de , E > de :: EnumAccess < 'de > for StringDeserializer < E > where E : de :: Error , { type Error = E ; type Variant = private :: UnitOnly < E > ; fn variant_seed < T > (self , seed : T) -> Result < (T :: Value , Self :: Variant) , Self :: Error > where T : de :: DeserializeSeed < 'de > , { seed . deserialize (self) . map (private :: unit_only) } }
};
}
