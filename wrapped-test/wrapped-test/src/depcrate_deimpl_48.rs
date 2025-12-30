// Generated macro for impl_48 (impl)
macro_rules! Depcrate_deimpl_48 {
() => {
// Module: crate::de
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'de , 'a > SeqAccess < 'de > for DeserializerSeqVisitor < 'a , 'de > { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Error > where T : DeserializeSeed < 'de > , { if self . de . peek_token_opt () == Some (self . end) { return Ok (None) ; } self . len = self . len . map (| len | len . saturating_sub (1)) ; seed . deserialize (& mut * self . de) . map (Some) } fn size_hint (& self) -> Option < usize > { self . len } }
};
}
