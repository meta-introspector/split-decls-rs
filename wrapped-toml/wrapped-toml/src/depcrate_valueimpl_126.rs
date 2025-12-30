// Generated macro for impl_126 (impl)
macro_rules! Depcrate_valueimpl_126 {
() => {
// Module: crate::value
// Provides: {"impl_126"}
// Dependencies: {}
impl ser :: SerializeTupleStruct for ValueSerializeVec { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value , crate :: ser :: Error > { ser :: SerializeSeq :: end (self) } }
};
}
