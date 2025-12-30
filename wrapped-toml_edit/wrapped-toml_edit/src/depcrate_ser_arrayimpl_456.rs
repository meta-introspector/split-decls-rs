// Generated macro for impl_456 (impl)
macro_rules! Depcrate_ser_arrayimpl_456 {
() => {
// Module: crate::ser::array
// Provides: {"impl_456"}
// Dependencies: {}
impl serde_core :: ser :: SerializeSeq for SerializeValueArray { type Ok = crate :: Value ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { let value = value . serialize (super :: ValueSerializer { }) ? ; self . values . push (crate :: Item :: Value (value)) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (crate :: Value :: Array (crate :: Array :: with_vec (self . values))) } }
};
}
