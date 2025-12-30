// Generated macro for impl_46 (impl)
macro_rules! Depcrate_mapimpl_46 {
() => {
// Module: crate::map
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'access , 'de > MapAccess < 'de > for Map < 'access , 'de > { type Error = Error ; fn next_key_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : DeserializeSeed < 'de > , { self . erased . erased_next_key_seed (& mut Some (seed)) . map (| erased_value | match erased_value { Some (value) => Some (unsafe { ErasedValue :: take :: < T :: Value > (value) }) , None => None , }) } fn next_value_seed < T > (& mut self , seed : T) -> Result < T :: Value , Self :: Error > where T : DeserializeSeed < 'de > , { self . erased . erased_next_value_seed (& mut Some (seed)) . map (| erased_value | unsafe { ErasedValue :: take :: < T :: Value > (erased_value) }) } fn size_hint (& self) -> Option < usize > { self . erased . erased_size_hint () } }
};
}
