macro_rules! deps {
    () => {
        Value!();
        NameDeserializer!();
        MapKeyDeserializer!();
        DeserializerError!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 'de > serde :: Deserializer < 'de > for MapKeyDeserializer { type Error = DeserializerError ; # [inline] fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , DeserializerError > where V : Visitor < 'de > , { NameDeserializer :: new (self . key) . deserialize_any (visitor) } # [inline] fn deserialize_enum < V > (self , name : & 'static str , variants : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , DeserializerError > where V : Visitor < 'de > , { self . key . into_deserializer () . deserialize_enum (name , variants , visitor) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes byte_buf unit unit_struct seq tuple option newtype_struct tuple_struct map struct identifier ignored_any } }
    };
}

impl_23!()