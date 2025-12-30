// Generated macro for impl_107 (impl)
macro_rules! Depcrate_de_valueimpl_107 {
() => {
// Module: crate::de::value
// Provides: {"impl_107"}
// Dependencies: {}
impl < I , E > SeqDeserializer < I , E > where I : Iterator , E : de :: Error , { # [doc = " Check for remaining elements after passing a `SeqDeserializer` to"] # [doc = " `Visitor::visit_seq`."] pub fn end (self) -> Result < () , E > { let remaining = self . iter . count () ; if remaining == 0 { Ok (()) } else { Err (de :: Error :: invalid_length (self . count + remaining , & ExpectedInSeq (self . count) ,)) } } }
};
}
