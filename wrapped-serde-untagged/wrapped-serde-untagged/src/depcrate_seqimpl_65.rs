// Generated macro for impl_65 (impl)
macro_rules! Depcrate_seqimpl_65 {
() => {
// Module: crate::seq
// Provides: {"impl_65"}
// Dependencies: {}
impl < 'de , Access > ErasedSeqAccess < 'de > for Access where Access : SeqAccess < 'de > , { fn erased_next_element_seed (& mut self , seed : & mut dyn ErasedDeserializeSeed < 'de > ,) -> Result < Option < ErasedValue > , Error > { self . next_element_seed (seed) . map_err (error :: erase) } fn erased_size_hint (& self) -> Option < usize > { self . size_hint () } }
};
}
