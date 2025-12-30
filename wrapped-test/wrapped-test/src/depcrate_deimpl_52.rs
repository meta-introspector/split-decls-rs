// Generated macro for impl_52 (impl)
macro_rules! Depcrate_deimpl_52 {
() => {
// Module: crate::de
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'de , 'a > EnumAccess < 'de > for DeserializerEnumVisitor < 'a , 'de > { type Error = Error ; type Variant = Self ; fn variant_seed < V > (self , seed : V) -> Result < (V :: Value , Self) , Error > where V : DeserializeSeed < 'de > , { match self . de . peek_token () ? { Token :: UnitVariant { variant : v , .. } | Token :: NewtypeVariant { variant : v , .. } | Token :: TupleVariant { variant : v , .. } | Token :: StructVariant { variant : v , .. } => { let de = v . into_deserializer () ; let value = seed . deserialize (de) ? ; Ok ((value , self)) } _ => { let value = seed . deserialize (& mut * self . de) ? ; Ok ((value , self)) } } } }
};
}
