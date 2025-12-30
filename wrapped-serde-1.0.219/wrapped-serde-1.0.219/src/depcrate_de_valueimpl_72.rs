// Generated macro for impl_72 (impl)
macro_rules! Depcrate_de_valueimpl_72 {
() => {
// Module: crate::de::value
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'de , E > de :: Deserializer < 'de > for BorrowedStrDeserializer < 'de , E > where E : de :: Error , { type Error = E ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { visitor . visit_borrowed_str (self . value) } fn deserialize_enum < V > (self , name : & str , variants : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { let _ = name ; let _ = variants ; visitor . visit_enum (self) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct identifier ignored_any } }
};
}
