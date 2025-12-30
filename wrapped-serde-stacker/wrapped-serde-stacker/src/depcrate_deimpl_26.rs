// Generated macro for impl_26 (impl)
macro_rules! Depcrate_deimpl_26 {
() => {
// Module: crate::de
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'de , D > de :: MapAccess < 'de > for MapAccess < D > where D : de :: MapAccess < 'de > , { type Error = D :: Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , D :: Error > where K : de :: DeserializeSeed < 'de > , { self . delegate . next_key_seed (DeserializeSeed :: new (seed , self . param)) } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , D :: Error > where V : de :: DeserializeSeed < 'de > , { self . delegate . next_value_seed (DeserializeSeed :: new (seed , self . param)) } fn size_hint (& self) -> Option < usize > { self . delegate . size_hint () } }
};
}
