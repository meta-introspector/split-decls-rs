// Generated macro for impl_27 (impl)
macro_rules! Depcrate_deserializerimpl_27 {
() => {
// Module: crate::deserializer
// Provides: {"impl_27"}
// Dependencies: {}
impl < 'de > serde :: Deserializer < 'de > for MapDeserializer { type Error = DeserializerError ; # [inline] fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , DeserializerError > where V : Visitor < 'de > , { visitor . visit_map (self) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } }
};
}
