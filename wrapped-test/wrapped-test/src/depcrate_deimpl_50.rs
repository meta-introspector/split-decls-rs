// Generated macro for impl_50 (impl)
macro_rules! Depcrate_deimpl_50 {
() => {
// Module: crate::de
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'de , 'a > MapAccess < 'de > for DeserializerMapVisitor < 'a , 'de > { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Error > where K : DeserializeSeed < 'de > , { if self . de . peek_token_opt () == Some (self . end) { return Ok (None) ; } self . len = self . len . map (| len | len . saturating_sub (1)) ; seed . deserialize (& mut * self . de) . map (Some) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Error > where V : DeserializeSeed < 'de > , { seed . deserialize (& mut * self . de) } fn size_hint (& self) -> Option < usize > { self . len } }
};
}
