// Generated macro for impl_294 (impl)
macro_rules! Depcrate_serimpl_294 {
() => {
// Module: crate::ser
// Provides: {"impl_294"}
// Dependencies: {}
impl serde :: ser :: SerializeTupleStruct for SerializeTuple < '_ > { type Ok = Schema ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde :: Serialize + ? Sized , { serde :: ser :: SerializeTuple :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { serde :: ser :: SerializeTuple :: end (self) } }
};
}
