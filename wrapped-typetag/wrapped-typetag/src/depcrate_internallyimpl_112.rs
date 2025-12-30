// Generated macro for impl_112 (impl)
macro_rules! Depcrate_internallyimpl_112 {
() => {
// Module: crate::internally
// Provides: {"impl_112"}
// Dependencies: {}
impl < 'de , D > Deserializer < 'de > for StringKeyDeserializer < D > where D : Deserializer < 'de > , { type Error = D :: Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { self . delegate . deserialize_str (visitor) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } }
};
}
