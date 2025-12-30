// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
# [doc = " Forwarding impl to preserve context."] impl < 'a , 'b , 'de , X , F > de :: VariantAccess < 'de > for Wrap < 'a , 'b , X , F > where X : de :: VariantAccess < 'de > , F : FnMut (Path) , { type Error = X :: Error ; fn unit_variant (self) -> Result < () , X :: Error > { self . delegate . unit_variant () } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , X :: Error > where T : DeserializeSeed < 'de > , { let path = Path :: NewtypeVariant { parent : self . path } ; self . delegate . newtype_variant_seed (TrackedSeed :: new (seed , self . callback , path)) } fn tuple_variant < V > (self , len : usize , visitor : V) -> Result < V :: Value , X :: Error > where V : Visitor < 'de > , { self . delegate . tuple_variant (len , Wrap :: new (visitor , self . callback , self . path)) } fn struct_variant < V > (self , fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , X :: Error > where V : Visitor < 'de > , { self . delegate . struct_variant (fields , Wrap :: new (visitor , self . callback , self . path)) } }
};
}
