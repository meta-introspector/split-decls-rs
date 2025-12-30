// Generated macro for impl_23 (impl)
macro_rules! Depcrate_deimpl_23 {
() => {
// Module: crate::de
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'de , D > de :: SeqAccess < 'de > for SeqAccess < D > where D : de :: SeqAccess < 'de > , { type Error = D :: Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , D :: Error > where T : de :: DeserializeSeed < 'de > , { self . delegate . next_element_seed (DeserializeSeed :: new (seed , self . param)) } fn size_hint (& self) -> Option < usize > { self . delegate . size_hint () } }
};
}
