// Generated macro for impl_57 (impl)
macro_rules! Depcrate_deimpl_57 {
() => {
// Module: crate::de
// Provides: {"impl_57"}
// Dependencies: {}
impl < 'de , 'a > MapAccess < 'de > for EnumMapVisitor < 'a , 'de > { type Error = Error ; fn next_key_seed < K > (& mut self , seed : K) -> Result < Option < K :: Value > , Error > where K : DeserializeSeed < 'de > , { match self . variant . take () { Some (Token :: Str (variant)) => seed . deserialize (variant . into_deserializer ()) . map (Some) , Some (Token :: Bytes (variant)) => seed . deserialize (BytesDeserializer { value : variant }) . map (Some) , Some (Token :: U32 (variant)) => seed . deserialize (variant . into_deserializer ()) . map (Some) , Some (other) => Err (unexpected (other)) , None => Ok (None) , } } fn next_value_seed < V > (& mut self , seed : V) -> Result < V :: Value , Error > where V : DeserializeSeed < 'de > , { match self . format { EnumFormat :: Seq => { let value = { let visitor = DeserializerSeqVisitor { de : self . de , len : None , end : Token :: TupleVariantEnd , } ; seed . deserialize (SeqAccessDeserializer :: new (visitor)) ? } ; assert_next_token (self . de , Token :: TupleVariantEnd) ? ; Ok (value) } EnumFormat :: Map => { let value = { let visitor = DeserializerMapVisitor { de : self . de , len : None , end : Token :: StructVariantEnd , } ; seed . deserialize (MapAccessDeserializer :: new (visitor)) ? } ; assert_next_token (self . de , Token :: StructVariantEnd) ? ; Ok (value) } EnumFormat :: Any => seed . deserialize (& mut * self . de) , } } }
};
}
