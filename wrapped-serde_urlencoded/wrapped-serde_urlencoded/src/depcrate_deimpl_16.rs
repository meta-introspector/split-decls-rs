// Generated macro for impl_16 (impl)
macro_rules! Depcrate_deimpl_16 {
() => {
// Module: crate::de
// Provides: {"impl_16"}
// Dependencies: {}
impl < 'de > de :: Deserializer < 'de > for Deserializer < 'de > { type Error = Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { self . deserialize_map (visitor) } fn deserialize_map < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { visitor . visit_map (self . inner) } fn deserialize_seq < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { visitor . visit_seq (self . inner) } fn deserialize_unit < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { self . inner . end () ? ; visitor . visit_unit () } forward_to_deserialize_any ! { bool u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64 char str string option bytes byte_buf unit_struct newtype_struct tuple_struct struct identifier tuple enum ignored_any } }
};
}
