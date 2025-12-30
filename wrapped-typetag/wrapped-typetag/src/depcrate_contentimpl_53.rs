// Generated macro for impl_53 (impl)
macro_rules! Depcrate_contentimpl_53 {
() => {
// Module: crate::content
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'de , E > MapAccess < 'de > for MapDeserializer < 'de , E > where E : de :: Error , { type Error = E ; fn next_key_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : DeserializeSeed < 'de > , { match self . iter . next () { Some ((key , value)) => { self . value = Some (value) ; seed . deserialize (ContentDeserializer :: new (key)) . map (Some) } None => Ok (None) , } } fn next_value_seed < T > (& mut self , seed : T) -> Result < T :: Value , Self :: Error > where T : DeserializeSeed < 'de > , { match self . value . take () { Some (value) => seed . deserialize (ContentDeserializer :: new (value)) , None => Err (de :: Error :: custom ("value is missing")) , } } }
};
}
