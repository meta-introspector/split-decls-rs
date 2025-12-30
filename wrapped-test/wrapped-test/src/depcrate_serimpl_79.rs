// Generated macro for impl_79 (impl)
macro_rules! Depcrate_serimpl_79 {
() => {
// Module: crate::ser
// Provides: {"impl_79"}
// Dependencies: {}
impl < 'a > ser :: SerializeSeq for & mut Serializer < 'a > { type Ok = () ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Error > where T : ? Sized + Serialize , { value . serialize (& mut * * self) } fn end (self) -> Result < () , Error > { assert_next_token ! (self , SeqEnd) ; Ok (()) } }
};
}
