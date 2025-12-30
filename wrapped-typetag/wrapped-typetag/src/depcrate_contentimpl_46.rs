// Generated macro for impl_46 (impl)
macro_rules! Depcrate_contentimpl_46 {
() => {
// Module: crate::content
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'de , E > VariantAccess < 'de > for VariantDeserializer < 'de , E > where E : de :: Error , { type Error = E ; fn unit_variant (self) -> Result < () , E > { match self . value { Some (value) => Deserialize :: deserialize (ContentDeserializer :: new (value)) , None => Ok (()) , } } fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , E > where T : DeserializeSeed < 'de > , { match self . value { Some (value) => seed . deserialize (ContentDeserializer :: new (value)) , None => Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "newtype variant" ,)) , } } fn tuple_variant < V > (self , _len : usize , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { match self . value { Some (Content :: Seq (v)) => { Deserializer :: deserialize_any (SeqDeserializer :: new (v) , visitor) } Some (other) => Err (de :: Error :: invalid_type (other . unexpected () , & "tuple variant" ,)) , None => Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "tuple variant" ,)) , } } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { match self . value { Some (Content :: Map (v)) => { Deserializer :: deserialize_any (MapDeserializer :: new (v) , visitor) } Some (Content :: Seq (v)) => { Deserializer :: deserialize_any (SeqDeserializer :: new (v) , visitor) } Some (other) => Err (de :: Error :: invalid_type (other . unexpected () , & "struct variant" ,)) , None => Err (de :: Error :: invalid_type (Unexpected :: UnitVariant , & "struct variant" ,)) , } } }
};
}
