// Generated macro for impl_85 (impl)
macro_rules! Depcrate_serimpl_85 {
() => {
// Module: crate::ser
// Provides: {"impl_85"}
// Dependencies: {}
impl < 's , 'a > ser :: SerializeStructVariant for Variant < 's , 'a > { type Ok = () ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { key . serialize (& mut * self . ser) ? ; value . serialize (& mut * self . ser) } fn end (self) -> Result < () , Self :: Error > { match self . end { Token :: StructVariantEnd => assert_next_token ! (self . ser , StructVariantEnd) , Token :: MapEnd => assert_next_token ! (self . ser , MapEnd) , _ => unreachable ! () , } Ok (()) } }
};
}
