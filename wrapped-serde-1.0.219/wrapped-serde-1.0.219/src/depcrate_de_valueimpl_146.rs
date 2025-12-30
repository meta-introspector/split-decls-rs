// Generated macro for impl_146 (impl)
macro_rules! Depcrate_de_valueimpl_146 {
() => {
// Module: crate::de::value
// Provides: {"impl_146"}
// Dependencies: {}
impl < 'de , A > de :: Deserializer < 'de > for EnumAccessDeserializer < A > where A : de :: EnumAccess < 'de > , { type Error = A :: Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { visitor . visit_enum (self . access) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } }
};
}
