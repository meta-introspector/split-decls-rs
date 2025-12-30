// Generated macro for impl_22 (impl)
macro_rules! Depcrate_deserializerimpl_22 {
() => {
// Module: crate::deserializer
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'de > serde :: Deserializer < 'de > for SeqDeserializer { type Error = DeserializerError ; # [inline] fn deserialize_any < V > (mut self , visitor : V) -> Result < V :: Value , DeserializerError > where V : Visitor < 'de > , { let len = self . iter . len () ; if len == 0 { visitor . visit_unit () } else { let ret = visitor . visit_seq (& mut self) ? ; let remaining = self . iter . len () ; if remaining == 0 { Ok (ret) } else { Err (DeserializerError :: invalid_length (len , & "fewer elements in array" ,)) } } } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } }
};
}
