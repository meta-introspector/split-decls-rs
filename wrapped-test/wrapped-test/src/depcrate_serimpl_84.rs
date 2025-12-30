// Generated macro for impl_84 (impl)
macro_rules! Depcrate_serimpl_84 {
() => {
// Module: crate::ser
// Provides: {"impl_84"}
// Dependencies: {}
impl < 'a > ser :: SerializeStruct for & mut Serializer < 'a > { type Ok = () ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { key . serialize (& mut * * self) ? ; value . serialize (& mut * * self) } fn end (self) -> Result < () , Self :: Error > { assert_next_token ! (self , StructEnd) ; Ok (()) } }
};
}
