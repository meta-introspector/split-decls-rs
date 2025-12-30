// Generated macro for impl_457 (impl)
macro_rules! Depcrate_ser_arrayimpl_457 {
() => {
// Module: crate::ser::array
// Provides: {"impl_457"}
// Dependencies: {}
impl serde_core :: ser :: SerializeTuple for SerializeValueArray { type Ok = crate :: Value ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { serde_core :: ser :: SerializeSeq :: end (self) } }
};
}
