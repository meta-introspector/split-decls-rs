// Generated macro for ErasedSeqAccess (trait)
macro_rules! Depcrate_seqErasedSeqAccess {
() => {
// Module: crate::seq
// Provides: {"ErasedSeqAccess"}
// Dependencies: {}
trait ErasedSeqAccess < 'de > { fn erased_next_element_seed (& mut self , seed : & mut dyn ErasedDeserializeSeed < 'de > ,) -> Result < Option < ErasedValue > , Error > ; fn erased_size_hint (& self) -> Option < usize > ; }
};
}
