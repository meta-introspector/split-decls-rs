// Generated macro for impl_508 (impl)
macro_rules! Depcrate_ser_value_arrayimpl_508 {
() => {
// Module: crate::ser::value::array
// Provides: {"impl_508"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeTupleStruct for SerializeValueArray < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { serde_core :: ser :: SerializeSeq :: end (self) } }
};
}
