// Generated macro for impl_128 (impl)
macro_rules! Depcrate_de_valueimpl_128 {
() => {
// Module: crate::de::value
// Provides: {"impl_128"}
// Dependencies: {}
impl < 'de , I , E > de :: SeqAccess < 'de > for MapDeserializer < 'de , I , E > where I : Iterator , I :: Item : private :: Pair , First < I :: Item > : IntoDeserializer < 'de , E > , Second < I :: Item > : IntoDeserializer < 'de , E > , E : de :: Error , { type Error = E ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : de :: DeserializeSeed < 'de > , { match self . next_pair () { Some ((k , v)) => { let de = PairDeserializer (k , v , PhantomData) ; seed . deserialize (de) . map (Some) } None => Ok (None) , } } fn size_hint (& self) -> Option < usize > { size_hint :: from_bounds (& self . iter) } }
};
}
