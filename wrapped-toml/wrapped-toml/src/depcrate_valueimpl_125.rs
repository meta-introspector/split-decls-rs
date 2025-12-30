// Generated macro for impl_125 (impl)
macro_rules! Depcrate_valueimpl_125 {
() => {
// Module: crate::value
// Provides: {"impl_125"}
// Dependencies: {}
impl ser :: SerializeTuple for ValueSerializeVec { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeSeq :: serialize_element (self , value) } fn end (self) -> Result < Value , crate :: ser :: Error > { ser :: SerializeSeq :: end (self) } }
};
}
