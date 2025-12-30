// Generated macro for impl_24 (impl)
macro_rules! Depcrate_deimpl_24 {
() => {
// Module: crate::de
// Provides: {"impl_24"}
// Dependencies: {}
impl < 'de > de :: EnumAccess < 'de > for ValueEnumAccess < 'de > { type Error = Error ; type Variant = UnitOnlyVariantAccess ; fn variant_seed < V > (self , seed : V ,) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : de :: DeserializeSeed < 'de > , { let variant = seed . deserialize (self . 0 . into_deserializer ()) ? ; Ok ((variant , UnitOnlyVariantAccess)) } }
};
}
