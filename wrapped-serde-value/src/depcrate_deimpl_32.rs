// Generated macro for impl_32 (impl)
macro_rules! Depcrate_deimpl_32 {
() => {
// Module: crate::de
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'de > de :: Deserializer < 'de > for Value { type Error = DeserializerError ; fn deserialize_any < V : de :: Visitor < 'de > > (self , visitor : V) -> Result < V :: Value , Self :: Error > { ValueDeserializer :: new (self) . deserialize_any (visitor) } fn deserialize_option < V : de :: Visitor < 'de > > (self , visitor : V) -> Result < V :: Value , Self :: Error > { ValueDeserializer :: new (self) . deserialize_option (visitor) } fn deserialize_enum < V : de :: Visitor < 'de > > (self , name : & 'static str , variants : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > { ValueDeserializer :: new (self) . deserialize_enum (name , variants , visitor) } fn deserialize_newtype_struct < V : de :: Visitor < 'de > > (self , name : & 'static str , visitor : V ,) -> Result < V :: Value , Self :: Error > { ValueDeserializer :: new (self) . deserialize_newtype_struct (name , visitor) } forward_to_deserialize_any ! { bool u8 u16 u32 u64 i8 i16 i32 i64 f32 f64 char str string unit seq bytes byte_buf map unit_struct tuple_struct struct tuple ignored_any identifier } }
};
}
