// Generated macro for impl_22 (impl)
macro_rules! Depcrate_deimpl_22 {
() => {
// Module: crate::de
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'de > de :: Deserializer < 'de > for Part < 'de > { type Error = Error ; fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { match self . 0 { Cow :: Borrowed (value) => visitor . visit_borrowed_str (value) , Cow :: Owned (value) => visitor . visit_string (value) , } } fn deserialize_option < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { visitor . visit_some (self) } fn deserialize_enum < V > (self , _name : & 'static str , _variants : & 'static [& 'static str] , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { visitor . visit_enum (ValueEnumAccess (self . 0)) } fn deserialize_newtype_struct < V > (self , _name : & 'static str , visitor : V ,) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { visitor . visit_newtype_struct (self) } forward_to_deserialize_any ! { char str string unit bytes byte_buf unit_struct tuple_struct struct identifier tuple ignored_any seq map } forward_parsed_value ! { bool => deserialize_bool , u8 => deserialize_u8 , u16 => deserialize_u16 , u32 => deserialize_u32 , u64 => deserialize_u64 , u128 => deserialize_u128 , i8 => deserialize_i8 , i16 => deserialize_i16 , i32 => deserialize_i32 , i64 => deserialize_i64 , i128 => deserialize_i128 , f32 => deserialize_f32 , f64 => deserialize_f64 , } }
};
}
