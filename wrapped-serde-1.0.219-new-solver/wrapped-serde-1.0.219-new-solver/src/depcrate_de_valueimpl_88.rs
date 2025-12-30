// Generated macro for impl_88 (impl)
macro_rules! Depcrate_de_valueimpl_88 {
() => {
// Module: crate::de::value
// Provides: {"impl_88"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl < 'de , 'a , E > de :: Deserializer < 'de > for CowStrDeserializer < 'a , E > where E : de :: Error , { type Error = E ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { match self . value { Cow :: Borrowed (string) => visitor . visit_str (string) , Cow :: Owned (string) => visitor . visit_string (string) , } } fn deserialize_enum < V > (self , name : & str , variants : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { let _ = name ; let _ = variants ; visitor . visit_enum (self) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct identifier ignored_any } }
};
}
