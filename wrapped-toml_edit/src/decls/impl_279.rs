macro_rules! deps {
    () => {
        Deserializer!();
        ArrayDeserializer!();
        Value!();
        Error!();
        ArraySeqAccess!();
    };
}

macro_rules! impl_279 {
    () => {
        deps!();
        impl < 'de > serde_core :: Deserializer < 'de > for ArrayDeserializer { type Error = Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : serde_core :: de :: Visitor < 'de > , { visitor . visit_seq (ArraySeqAccess :: new (self . input)) } fn deserialize_struct < V > (self , name : & 'static str , _fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Error > where V : serde_core :: de :: Visitor < 'de > , { if serde_spanned :: de :: is_spanned (name) { if let Some (span) = self . span . clone () { return visitor . visit_map (serde_spanned :: de :: SpannedDeserializer :: < Self , Error > :: new (self , span) ,) ; } else { return Err (Error :: custom ("value is missing a span" , None)) ; } } self . deserialize_any (visitor) } serde_core :: forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq bytes byte_buf map option unit newtype_struct ignored_any unit_struct tuple_struct tuple enum identifier } }
    };
}

impl_279!();