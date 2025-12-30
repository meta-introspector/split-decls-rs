// Generated macro for impl_107 (impl)
macro_rules! Depcrate_internallyimpl_107 {
() => {
// Module: crate::internally
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'de , A > EnumAccess < 'de > for MapEntryAsEnum < A > where A : MapAccess < 'de > , { type Error = A :: Error ; type Variant = Self ; fn variant_seed < V > (mut self , seed : V) -> Result < (V :: Value , Self :: Variant) , Self :: Error > where V : DeserializeSeed < 'de > , { match self . map . next_key_seed (seed) ? { Some (variant) => Ok ((variant , self)) , None => Err (de :: Error :: custom (format_args ! ("expected enum {}" , self . name))) , } } }
};
}
