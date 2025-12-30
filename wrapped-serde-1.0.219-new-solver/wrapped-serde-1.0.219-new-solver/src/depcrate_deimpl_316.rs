// Generated macro for impl_316 (impl)
macro_rules! Depcrate_deimpl_316 {
() => {
// Module: crate::de
// Provides: {"impl_316"}
// Dependencies: {}
impl < 'de , A > SeqAccess < 'de > for & mut A where A : ? Sized + SeqAccess < 'de > , { type Error = A :: Error ; # [inline] fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : DeserializeSeed < 'de > , { (* * self) . next_element_seed (seed) } # [inline] fn next_element < T > (& mut self) -> Result < Option < T > , Self :: Error > where T : Deserialize < 'de > , { (* * self) . next_element () } # [inline] fn size_hint (& self) -> Option < usize > { (* * self) . size_hint () } }
};
}
