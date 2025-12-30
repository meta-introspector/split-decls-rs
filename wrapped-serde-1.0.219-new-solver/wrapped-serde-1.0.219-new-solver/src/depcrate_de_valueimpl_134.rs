// Generated macro for impl_134 (impl)
macro_rules! Depcrate_de_valueimpl_134 {
() => {
// Module: crate::de::value
// Provides: {"impl_134"}
// Dependencies: {}
impl < 'de , A , B , E > de :: SeqAccess < 'de > for PairVisitor < A , B , E > where A : IntoDeserializer < 'de , E > , B : IntoDeserializer < 'de , E > , E : de :: Error , { type Error = E ; fn next_element_seed < T > (& mut self , seed : T) -> Result < Option < T :: Value > , Self :: Error > where T : de :: DeserializeSeed < 'de > , { if let Some (k) = self . 0 . take () { seed . deserialize (k . into_deserializer ()) . map (Some) } else if let Some (v) = self . 1 . take () { seed . deserialize (v . into_deserializer ()) . map (Some) } else { Ok (None) } } fn size_hint (& self) -> Option < usize > { if self . 0 . is_some () { Some (2) } else if self . 1 . is_some () { Some (1) } else { Some (0) } } }
};
}
