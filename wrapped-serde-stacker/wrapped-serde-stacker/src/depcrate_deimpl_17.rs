// Generated macro for impl_17 (impl)
macro_rules! Depcrate_deimpl_17 {
() => {
// Module: crate::de
// Provides: {"impl_17"}
// Dependencies: {}
impl < 'de , D > de :: VariantAccess < 'de > for VariantAccess < D > where D : de :: VariantAccess < 'de > , { type Error = D :: Error ; fn unit_variant (self) -> Result < () , D :: Error > { self . delegate . unit_variant () } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , D :: Error > where T : de :: DeserializeSeed < 'de > , { self . delegate . newtype_variant_seed (DeserializeSeed :: new (seed , self . param)) } fn tuple_variant < V > (self , len : usize , visitor : V) -> Result < V :: Value , D :: Error > where V : de :: Visitor < 'de > , { self . delegate . tuple_variant (len , Visitor :: new (visitor , self . param)) } fn struct_variant < V > (self , fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , D :: Error > where V : de :: Visitor < 'de > , { self . delegate . struct_variant (fields , Visitor :: new (visitor , self . param)) } }
};
}
