// Generated macro for impl_81 (impl)
macro_rules! Depcrate_serimpl_81 {
() => {
// Module: crate::ser
// Provides: {"impl_81"}
// Dependencies: {}
impl < 'a > ser :: SerializeTupleStruct for & mut Serializer < 'a > { type Ok = () ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Error > where T : ? Sized + Serialize , { value . serialize (& mut * * self) } fn end (self) -> Result < () , Error > { assert_next_token ! (self , TupleStructEnd) ; Ok (()) } }
};
}
