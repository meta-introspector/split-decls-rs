// Generated macro for impl_82 (impl)
macro_rules! Depcrate_serimpl_82 {
() => {
// Module: crate::ser
// Provides: {"impl_82"}
// Dependencies: {}
impl < 's , 'a > ser :: SerializeTupleVariant for Variant < 's , 'a > { type Ok = () ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Error > where T : ? Sized + Serialize , { value . serialize (& mut * self . ser) } fn end (self) -> Result < () , Error > { match self . end { Token :: TupleVariantEnd => assert_next_token ! (self . ser , TupleVariantEnd) , Token :: SeqEnd => assert_next_token ! (self . ser , SeqEnd) , _ => unreachable ! () , } Ok (()) } }
};
}
