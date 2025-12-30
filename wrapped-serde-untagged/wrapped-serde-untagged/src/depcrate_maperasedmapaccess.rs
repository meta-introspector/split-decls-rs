// Generated macro for ErasedMapAccess (trait)
macro_rules! Depcrate_mapErasedMapAccess {
() => {
// Module: crate::map
// Provides: {"ErasedMapAccess"}
// Dependencies: {}
trait ErasedMapAccess < 'de > { fn erased_next_key_seed (& mut self , seed : & mut dyn ErasedDeserializeSeed < 'de > ,) -> Result < Option < ErasedValue > , Error > ; fn erased_next_value_seed (& mut self , seed : & mut dyn ErasedDeserializeSeed < 'de > ,) -> Result < ErasedValue , Error > ; fn erased_size_hint (& self) -> Option < usize > ; }
};
}
