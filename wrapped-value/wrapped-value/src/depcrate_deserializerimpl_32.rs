// Generated macro for impl_32 (impl)
macro_rules! Depcrate_deserializerimpl_32 {
() => {
// Module: crate::deserializer
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'de > de :: Deserializer < 'de > for NameDeserializer { type Error = DeserializerError ; # [inline] fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , DeserializerError > where V : de :: Visitor < 'de > , { visitor . visit_string (self . value . to_string ()) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple enum tuple_struct map struct identifier ignored_any } }
};
}
