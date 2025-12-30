// Generated macro for impl_80 (impl)
macro_rules! Depcrate_serimpl_80 {
() => {
// Module: crate::ser
// Provides: {"impl_80"}
// Dependencies: {}
impl < 'a > ser :: SerializeTuple for & mut Serializer < 'a > { type Ok = () ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Error > where T : ? Sized + Serialize , { value . serialize (& mut * * self) } fn end (self) -> Result < () , Error > { assert_next_token ! (self , TupleEnd) ; Ok (()) } }
};
}
