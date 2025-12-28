macro_rules! deps {
    () => {
        Deserializer!();
        Table!();
        Value!();
        Error!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl < 'de > de :: Deserializer < 'de > for Table { type Error = crate :: de :: Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , crate :: de :: Error > where V : de :: Visitor < 'de > , { Value :: Table (self) . deserialize_any (visitor) } # [inline] fn deserialize_enum < V > (self , name : & 'static str , variants : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , crate :: de :: Error > where V : de :: Visitor < 'de > , { Value :: Table (self) . deserialize_enum (name , variants , visitor) } fn deserialize_option < V > (self , visitor : V) -> Result < V :: Value , crate :: de :: Error > where V : de :: Visitor < 'de > , { Value :: Table (self) . deserialize_option (visitor) } fn deserialize_newtype_struct < V > (self , name : & 'static str , visitor : V ,) -> Result < V :: Value , crate :: de :: Error > where V : de :: Visitor < 'de > , { Value :: Table (self) . deserialize_newtype_struct (name , visitor) } serde_core :: forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq bytes byte_buf map unit_struct tuple_struct struct tuple ignored_any identifier } }
    };
}

impl_377!();