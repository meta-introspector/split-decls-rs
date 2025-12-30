// Generated macro for impl_123 (impl)
macro_rules! Depcrate_de_valueimpl_123 {
() => {
// Module: crate::de::value
// Provides: {"impl_123"}
// Dependencies: {}
impl < 'de , I , E > MapDeserializer < 'de , I , E > where I : Iterator , I :: Item : private :: Pair , E : de :: Error , { # [doc = " Check for remaining elements after passing a `MapDeserializer` to"] # [doc = " `Visitor::visit_map`."] pub fn end (self) -> Result < () , E > { let remaining = self . iter . count () ; if remaining == 0 { Ok (()) } else { Err (de :: Error :: invalid_length (self . count + remaining , & ExpectedInMap (self . count) ,)) } } }
};
}
