// Generated macro for impl_19 (impl)
macro_rules! Depcrate_deserializerimpl_19 {
() => {
// Module: crate::deserializer
// Provides: {"impl_19"}
// Dependencies: {}
impl < 'de > VariantAccess < 'de > for VariantDeserializer { type Error = DeserializerError ; # [inline] fn unit_variant (self) -> Result < () , DeserializerError > { match self . value { Some (value) => Deserialize :: deserialize (value) , None => Ok (()) , } } # [inline] fn newtype_variant_seed < T > (self , seed : T) -> Result < T :: Value , DeserializerError > where T : DeserializeSeed < 'de > , { match self . value { Some (value) => seed . deserialize (value) , None => Err (DeserializerError :: invalid_type (Unexpected :: UnitVariant , & "newtype variant" ,)) , } } fn tuple_variant < V > (self , _len : usize , visitor : V) -> Result < V :: Value , DeserializerError > where V : Visitor < 'de > , { match self . value { Some (ConstValue :: List (v)) => { serde :: Deserializer :: deserialize_any (SeqDeserializer :: new (v) , visitor) } Some (other) => Err (serde :: de :: Error :: invalid_type (other . unexpected () , & "tuple variant" ,)) , None => Err (DeserializerError :: invalid_type (Unexpected :: UnitVariant , & "tuple variant" ,)) , } } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , DeserializerError > where V : Visitor < 'de > , { match self . value { Some (ConstValue :: Object (v)) => { serde :: Deserializer :: deserialize_any (MapDeserializer :: new (v) , visitor) } Some (other) => Err (DeserializerError :: invalid_type (other . unexpected () , & "struct variant" ,)) , None => Err (DeserializerError :: invalid_type (Unexpected :: UnitVariant , & "struct variant" ,)) , } } }
};
}
