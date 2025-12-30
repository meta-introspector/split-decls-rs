// Generated macro for impl_52 (impl)
macro_rules! Depcrate_serimpl_52 {
() => {
// Module: crate::ser
// Provides: {"impl_52"}
// Dependencies: {}
impl ser :: SerializeSeq for SerializeSeq { type Ok = Value ; type Error = SerializerError ; fn serialize_element < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize , { let value = value . serialize (Serializer) ? ; self . 0 . push (value) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (Value :: Seq (self . 0)) } }
};
}
