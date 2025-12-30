// Generated macro for impl_26 (impl)
macro_rules! Depcrate_deimpl_26 {
() => {
// Module: crate::de
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'de > de :: VariantAccess < 'de > for UnitOnlyVariantAccess { type Error = Error ; fn unit_variant (self) -> Result < () , Self :: Error > { Ok (()) } fn newtype_variant_seed < T > (self , _seed : T) -> Result < T :: Value , Self :: Error > where T : de :: DeserializeSeed < 'de > , { Err (Error :: custom ("expected unit variant")) } fn tuple_variant < V > (self , _len : usize , _visitor : V ,) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { Err (Error :: custom ("expected unit variant")) } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , _visitor : V ,) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { Err (Error :: custom ("expected unit variant")) } }
};
}
