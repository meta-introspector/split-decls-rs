// Generated macro for impl_47 (impl)
macro_rules! Depcrate_mapimpl_47 {
() => {
// Module: crate::map
// Provides: {"impl_47"}
// Dependencies: {}
impl < 'de , Access > ErasedMapAccess < 'de > for Access where Access : MapAccess < 'de > , { fn erased_next_key_seed (& mut self , seed : & mut dyn ErasedDeserializeSeed < 'de > ,) -> Result < Option < ErasedValue > , Error > { self . next_key_seed (seed) . map_err (error :: erase) } fn erased_next_value_seed (& mut self , seed : & mut dyn ErasedDeserializeSeed < 'de > ,) -> Result < ErasedValue , Error > { self . next_value_seed (seed) . map_err (error :: erase) } fn erased_size_hint (& self) -> Option < usize > { self . size_hint () } }
};
}
