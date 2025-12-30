// Generated macro for impl_124 (impl)
macro_rules! Depcrate_valueimpl_124 {
() => {
// Module: crate::value
// Provides: {"impl_124"}
// Dependencies: {}
impl ser :: SerializeSeq for ValueSerializeVec { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { self . vec . push (Value :: try_from (value) ?) ; Ok (()) } fn end (self) -> Result < Value , crate :: ser :: Error > { Ok (Value :: Array (self . vec)) } }
};
}
