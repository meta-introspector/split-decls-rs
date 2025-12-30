// Generated macro for impl_96 (impl)
macro_rules! Depcrate_de_valueimpl_96 {
() => {
// Module: crate::de::value
// Provides: {"impl_96"}
// Dependencies: {}
impl < 'de , 'a , E > Deserializer < 'de > for BytesDeserializer < 'a , E > where E : de :: Error , { type Error = E ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { visitor . visit_bytes (self . value) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } }
};
}
