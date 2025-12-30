// Generated macro for impl_105 (impl)
macro_rules! Depcrate_internallyimpl_105 {
() => {
// Module: crate::internally
// Provides: {"impl_105"}
// Dependencies: {}
impl < 'de , A > MapAccess < 'de > for MapWithStringKeys < A > where A : MapAccess < 'de > , { type Error = A :: Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Self :: Error > where K : DeserializeSeed < 'de > , { self . map . next_key_seed (StringKeySeed { seed }) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Self :: Error > where V : DeserializeSeed < 'de > , { self . map . next_value_seed (seed) } }
};
}
