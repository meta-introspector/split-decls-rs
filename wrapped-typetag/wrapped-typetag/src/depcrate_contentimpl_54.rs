// Generated macro for impl_54 (impl)
macro_rules! Depcrate_contentimpl_54 {
() => {
// Module: crate::content
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'de , E > Deserializer < 'de > for MapDeserializer < 'de , E > where E : de :: Error , { type Error = E ; # [inline] fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : Visitor < 'de > , { visitor . visit_map (self) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } }
};
}
