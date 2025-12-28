macro_rules! deps {
    () => {
        Error!();
        Value!();
        Deserializer!();
        TableMapAccess!();
        TableDeserializer!();
    };
}

macro_rules! impl_136 {
    () => {
        deps!();
        impl < 'de > serde_core :: Deserializer < 'de > for TableDeserializer < 'de > { type Error = Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : serde_core :: de :: Visitor < 'de > , { visitor . visit_map (TableMapAccess :: new (self)) } fn deserialize_option < V > (self , visitor : V) -> Result < V :: Value , Error > where V : serde_core :: de :: Visitor < 'de > , { visitor . visit_some (self) } fn deserialize_newtype_struct < V > (self , _name : & 'static str , visitor : V ,) -> Result < V :: Value , Error > where V : serde_core :: de :: Visitor < 'de > , { visitor . visit_newtype_struct (self) } fn deserialize_struct < V > (self , name : & 'static str , _fields : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Error > where V : serde_core :: de :: Visitor < 'de > , { if serde_spanned :: de :: is_spanned (name) { let span = self . span . clone () ; return visitor . visit_map (super :: SpannedDeserializer :: new (self , span)) ; } self . deserialize_any (visitor) } fn deserialize_enum < V > (self , _name : & 'static str , _variants : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Error > where V : serde_core :: de :: Visitor < 'de > , { if self . items . is_empty () { Err (Error :: custom ("wanted exactly 1 element, found 0 elements" , Some (self . span) ,)) } else if self . items . len () != 1 { Err (Error :: custom ("wanted exactly 1 element, more than 1 element" , Some (self . span) ,)) } else { visitor . visit_enum (TableMapAccess :: new (self)) } } serde_core :: forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string seq bytes byte_buf map unit ignored_any unit_struct tuple_struct tuple identifier } }
    };
}

impl_136!();