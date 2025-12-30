// Generated macro for impl_511 (impl)
macro_rules! Depcrate_ser_value_arrayimpl_511 {
() => {
// Module: crate::ser::value::array
// Provides: {"impl_511"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeTupleVariant for SerializeTupleVariant < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeSeq :: serialize_element (& mut self . inner , value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { let dst = self . inner . end () ? ; dst . space () ? ; dst . close_inline_table () ? ; Ok (dst) } }
};
}
