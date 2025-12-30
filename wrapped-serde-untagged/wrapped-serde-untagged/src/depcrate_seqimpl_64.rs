// Generated macro for impl_64 (impl)
macro_rules! Depcrate_seqimpl_64 {
() => {
// Module: crate::seq
// Provides: {"impl_64"}
// Dependencies: {}
impl < 'access , 'de > SeqAccess < 'de > for Seq < 'access , 'de > { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : DeserializeSeed < 'de > , { self . erased . erased_next_element_seed (& mut Some (seed)) . map (| erased_value | match erased_value { Some (value) => Some (unsafe { ErasedValue :: take :: < T :: Value > (value) }) , None => None , }) } fn size_hint (& self) -> Option < usize > { self . erased . erased_size_hint () } }
};
}
