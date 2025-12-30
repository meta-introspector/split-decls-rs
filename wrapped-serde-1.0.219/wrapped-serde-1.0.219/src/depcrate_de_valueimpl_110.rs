// Generated macro for impl_110 (impl)
macro_rules! Depcrate_de_valueimpl_110 {
() => {
// Module: crate::de::value
// Provides: {"impl_110"}
// Dependencies: {}
impl < 'de , I , T , E > de :: SeqAccess < 'de > for SeqDeserializer < I , E > where I : Iterator < Item = T > , T : IntoDeserializer < 'de , E > , E : de :: Error , { type Error = E ; fn next_element_seed < V > (& mut self , seed : V) -> Result < Option < V :: Value > , Self :: Error > where V : de :: DeserializeSeed < 'de > , { match self . iter . next () { Some (value) => { self . count += 1 ; seed . deserialize (value . into_deserializer ()) . map (Some) } None => Ok (None) , } } fn size_hint (& self) -> Option < usize > { size_hint :: from_bounds (& self . iter) } }
};
}
