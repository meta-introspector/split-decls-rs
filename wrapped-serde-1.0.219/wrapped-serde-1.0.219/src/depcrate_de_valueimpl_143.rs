// Generated macro for impl_143 (impl)
macro_rules! Depcrate_de_valueimpl_143 {
() => {
// Module: crate::de::value
// Provides: {"impl_143"}
// Dependencies: {}
impl < 'de , A > de :: EnumAccess < 'de > for MapAccessDeserializer < A > where A : de :: MapAccess < 'de > , { type Error = A :: Error ; type Variant = private :: MapAsEnum < A > ; fn variant_seed < T > (mut self , seed : T) -> Result < (T :: Value , Self :: Variant) , Self :: Error > where T : de :: DeserializeSeed < 'de > , { match tri ! (self . map . next_key_seed (seed)) { Some (key) => Ok ((key , private :: map_as_enum (self . map))) , None => Err (de :: Error :: invalid_type (de :: Unexpected :: Map , & "enum")) , } } }
};
}
