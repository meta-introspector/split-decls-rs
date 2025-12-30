// Generated macro for impl_119 (impl)
macro_rules! Depcrate_de_valueimpl_119 {
() => {
// Module: crate::de::value
// Provides: {"impl_119"}
// Dependencies: {}
impl < 'de , A > de :: Deserializer < 'de > for SeqAccessDeserializer < A > where A : de :: SeqAccess < 'de > , { type Error = A :: Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { visitor . visit_seq (self . seq) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } }
};
}
