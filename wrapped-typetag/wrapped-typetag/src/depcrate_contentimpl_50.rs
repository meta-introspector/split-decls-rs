// Generated macro for impl_50 (impl)
macro_rules! Depcrate_contentimpl_50 {
() => {
// Module: crate::content
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'de , E > SeqAccess < 'de > for SeqDeserializer < 'de , E > where E : de :: Error , { type Error = E ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : DeserializeSeed < 'de > , { match self . iter . next () { Some (value) => seed . deserialize (ContentDeserializer :: new (value)) . map (Some) , None => Ok (None) , } } }
};
}
