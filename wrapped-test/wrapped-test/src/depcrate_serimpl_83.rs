// Generated macro for impl_83 (impl)
macro_rules! Depcrate_serimpl_83 {
() => {
// Module: crate::ser
// Provides: {"impl_83"}
// Dependencies: {}
impl < 'a > ser :: SerializeMap for & mut Serializer < 'a > { type Ok = () ; type Error = Error ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { key . serialize (& mut * * self) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { value . serialize (& mut * * self) } fn end (self) -> Result < () , Self :: Error > { assert_next_token ! (self , MapEnd) ; Ok (()) } }
};
}
