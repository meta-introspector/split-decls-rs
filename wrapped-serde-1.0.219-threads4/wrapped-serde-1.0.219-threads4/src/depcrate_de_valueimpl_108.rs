// Generated macro for impl_108 (impl)
macro_rules! Depcrate_de_valueimpl_108 {
() => {
// Module: crate::de::value
// Provides: {"impl_108"}
// Dependencies: {}
impl < 'de , I , T , E > de :: Deserializer < 'de > for SeqDeserializer < I , E > where I : Iterator < Item = T > , T : IntoDeserializer < 'de , E > , E : de :: Error , { type Error = E ; fn deserialize_any < V > (mut self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { let v = tri ! (visitor . visit_seq (& mut self)) ; tri ! (self . end ()) ; Ok (v) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any } }
};
}
