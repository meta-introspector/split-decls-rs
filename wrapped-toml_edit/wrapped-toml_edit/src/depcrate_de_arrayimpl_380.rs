// Generated macro for impl_380 (impl)
macro_rules! Depcrate_de_arrayimpl_380 {
() => {
// Module: crate::de::array
// Provides: {"impl_380"}
// Dependencies: {}
impl < 'de > serde_core :: de :: SeqAccess < 'de > for ArraySeqAccess { type Error = Error ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : serde_core :: de :: DeserializeSeed < 'de > , { match self . iter . next () { Some (v) => seed . deserialize (crate :: de :: ValueDeserializer :: new (v)) . map (Some) , None => Ok (None) , } } }
};
}
