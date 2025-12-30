// Generated macro for impl_127 (impl)
macro_rules! Depcrate_de_valueimpl_127 {
() => {
// Module: crate::de::value
// Provides: {"impl_127"}
// Dependencies: {}
impl < 'de , I , E > de :: MapAccess < 'de > for MapDeserializer < 'de , I , E > where I : Iterator , I :: Item : private :: Pair , First < I :: Item > : IntoDeserializer < 'de , E > , Second < I :: Item > : IntoDeserializer < 'de , E > , E : de :: Error , { type Error = E ; fn next_key_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : de :: DeserializeSeed < 'de > , { match self . next_pair () { Some ((key , value)) => { self . value = Some (value) ; seed . deserialize (key . into_deserializer ()) . map (Some) } None => Ok (None) , } } fn next_value_seed < T > (& mut self , seed : T) -> Result < T :: Value , Self :: Error > where T : de :: DeserializeSeed < 'de > , { let value = self . value . take () ; let value = value . expect ("MapAccess::next_value called before next_key") ; seed . deserialize (value . into_deserializer ()) } fn next_entry_seed < TK , TV > (& mut self , kseed : TK , vseed : TV ,) -> Result < Option < (TK :: Value , TV :: Value) > , Self :: Error > where TK : de :: DeserializeSeed < 'de > , TV : de :: DeserializeSeed < 'de > , { match self . next_pair () { Some ((key , value)) => { let key = tri ! (kseed . deserialize (key . into_deserializer ())) ; let value = tri ! (vseed . deserialize (value . into_deserializer ())) ; Ok (Some ((key , value))) } None => Ok (None) , } } fn size_hint (& self) -> Option < usize > { size_hint :: from_bounds (& self . iter) } }
};
}
