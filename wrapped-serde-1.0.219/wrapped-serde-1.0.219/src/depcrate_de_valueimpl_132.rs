// Generated macro for impl_132 (impl)
macro_rules! Depcrate_de_valueimpl_132 {
() => {
// Module: crate::de::value
// Provides: {"impl_132"}
// Dependencies: {}
impl < 'de , A , B , E > de :: Deserializer < 'de > for PairDeserializer < A , B , E > where A : IntoDeserializer < 'de , E > , B : IntoDeserializer < 'de , E > , E : de :: Error , { type Error = E ; forward_to_deserialize_any ! { bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf option unit unit_struct newtype_struct tuple_struct map struct enum identifier ignored_any } fn deserialize_any < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { self . deserialize_seq (visitor) } fn deserialize_seq < V > (self , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { let mut pair_visitor = PairVisitor (Some (self . 0) , Some (self . 1) , PhantomData) ; let pair = tri ! (visitor . visit_seq (& mut pair_visitor)) ; if pair_visitor . 1 . is_none () { Ok (pair) } else { let remaining = pair_visitor . size_hint () . unwrap () ; Err (de :: Error :: invalid_length (2 , & ExpectedInSeq (2 - remaining))) } } fn deserialize_tuple < V > (self , len : usize , visitor : V) -> Result < V :: Value , Self :: Error > where V : de :: Visitor < 'de > , { if len == 2 { self . deserialize_seq (visitor) } else { Err (de :: Error :: invalid_length (2 , & ExpectedInSeq (len))) } } }
};
}
