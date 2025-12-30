// Generated macro for impl_125 (impl)
macro_rules! Depcrate_de_valueimpl_125 {
() => {
// Module: crate::de::value
// Provides: {"impl_125"}
// Dependencies: {}
impl < 'de , I , E > de :: Deserializer < 'de > for MapDeserializer < 'de , I , E > where I : Iterator , I :: Item : private :: Pair , First < I :: Item > : IntoDeserializer < 'de , E > , Second < I :: Item > : IntoDeserializer < 'de , E > , E : de :: Error , { type Error = E ; fn deserialize_any < V > (mut self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { let value = tri ! (visitor . visit_map (& mut self)) ; tri ! (self . end ()) ; Ok (value) } fn deserialize_seq < V > (mut self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { let value = tri ! (visitor . visit_seq (& mut self)) ; tri ! (self . end ()) ; Ok (value) } fn deserialize_tuple < V > (self , len : usize , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { let _ = len ; self . deserialize_seq (visitor) } forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct tuple_struct map struct enum identifier ignored_any } }
};
}
