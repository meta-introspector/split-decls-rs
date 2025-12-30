// Generated macro for impl_23 (impl)
macro_rules! Depcrate_deserializerimpl_23 {
() => {
// Module: crate::deserializer
// Provides: {"impl_23"}
// Dependencies: {}
impl < 'de > SeqAccess < 'de > for SeqDeserializer { type Error = DeserializerError ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , DeserializerError > where T : DeserializeSeed < 'de > , { match self . iter . next () { Some (value) => seed . deserialize (value) . map (Some) , None => Ok (None) , } } # [inline] fn size_hint (& self) -> Option < usize > { match self . iter . size_hint () { (lower , Some (upper)) if lower == upper => Some (upper) , _ => None , } } }
};
}
