// Generated macro for impl_163 (impl)
macro_rules! Depcrate_de_deserializer_keyimpl_163 {
() => {
// Module: crate::de::deserializer::key
// Provides: {"impl_163"}
// Dependencies: {}
impl < 'de , E > serde_core :: de :: VariantAccess < 'de > for UnitOnly < E > where E : serde_core :: de :: Error , { type Error = E ; fn unit_variant (self) -> Result < () , Self :: Error > { Ok (()) } fn newtype_variant_seed < T > (self , _seed : T) -> Result < T :: Value , Self :: Error > where T : serde_core :: de :: DeserializeSeed < 'de > , { Err (serde_core :: de :: Error :: invalid_type (serde_core :: de :: Unexpected :: UnitVariant , & "newtype variant" ,)) } fn tuple_variant < V > (self , _len : usize , _visitor : V) -> Result < V :: Value , Self :: Error > where V : serde_core :: de :: Visitor < 'de > , { Err (serde_core :: de :: Error :: invalid_type (serde_core :: de :: Unexpected :: UnitVariant , & "tuple variant" ,)) } fn struct_variant < V > (self , _fields : & 'static [& 'static str] , _visitor : V ,) -> Result < V :: Value , Self :: Error > where V : serde_core :: de :: Visitor < 'de > , { Err (serde_core :: de :: Error :: invalid_type (serde_core :: de :: Unexpected :: UnitVariant , & "struct variant" ,)) } }
};
}
