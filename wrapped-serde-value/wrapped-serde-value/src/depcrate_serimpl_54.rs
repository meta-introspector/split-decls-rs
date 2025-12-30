// Generated macro for impl_54 (impl)
macro_rules! Depcrate_serimpl_54 {
() => {
// Module: crate::ser
// Provides: {"impl_54"}
// Dependencies: {}
impl ser :: SerializeTuple for SerializeTuple { type Ok = Value ; type Error = SerializerError ; fn serialize_element < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize , { let value = value . serialize (Serializer) ? ; self . 0 . push (value) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (Value :: Seq (self . 0)) } }
};
}
