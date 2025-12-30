// Generated macro for impl_108 (impl)
macro_rules! Depcrate_internallyimpl_108 {
() => {
// Module: crate::internally
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'de , A > VariantAccess < 'de > for MapEntryAsEnum < A > where A : MapAccess < 'de > , { type Error = A :: Error ; fn unit_variant (mut self) -> Result < () , Self :: Error > { self . map . next_value () } fn newtype_variant_seed < T > (mut self , seed : T) -> Result < T :: Value , Self :: Error > where T : DeserializeSeed < 'de > , { self . map . next_value_seed (seed) } fn tuple_variant < V > (mut self , len : usize , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { struct Wrap < V > { len : usize , visitor : V , } impl < 'de , V > DeserializeSeed < 'de > for Wrap < V > where V : Visitor < 'de > , { type Value = V :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_tuple (self . len , self . visitor) } } self . map . next_value_seed (Wrap { len , visitor }) } fn struct_variant < V > (mut self , fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { struct Wrap < V > { name : & 'static str , fields : & 'static [& 'static str] , visitor : V , } impl < 'de , V > DeserializeSeed < 'de > for Wrap < V > where V : Visitor < 'de > , { type Value = V :: Value ; fn deserialize < D > (self , deserializer : D) -> Result < Self :: Value , D :: Error > where D : Deserializer < 'de > , { deserializer . deserialize_struct (self . name , self . fields , self . visitor) } } self . map . next_value_seed (Wrap { name : self . name , fields , visitor , }) } }
};
}
