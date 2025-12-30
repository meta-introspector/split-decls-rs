// Generated macro for impl_151 (impl)
macro_rules! Depcrate_de_deserializer_arrayimpl_151 {
() => {
// Module: crate::de::deserializer::array
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'de > serde_core :: de :: SeqAccess < 'de > for ArraySeqAccess < 'de > { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : serde_core :: de :: DeserializeSeed < 'de > , { match self . iter . next () { Some (v) => { let span = v . span () ; let v = v . into_inner () ; seed . deserialize (crate :: de :: ValueDeserializer :: with_parts (v , span)) . map (Some) } None => Ok (None) , } } }
};
}
