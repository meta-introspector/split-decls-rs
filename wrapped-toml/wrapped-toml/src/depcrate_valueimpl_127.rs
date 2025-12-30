// Generated macro for impl_127 (impl)
macro_rules! Depcrate_valueimpl_127 {
() => {
// Module: crate::value
// Provides: {"impl_127"}
// Dependencies: {}
impl ser :: SerializeTupleVariant for ValueSerializeVec { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value , crate :: ser :: Error > { ser :: SerializeSeq :: end (self) } }
};
}
