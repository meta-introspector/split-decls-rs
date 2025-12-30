// Generated macro for impl_56 (impl)
macro_rules! Depcrate_serimpl_56 {
() => {
// Module: crate::ser
// Provides: {"impl_56"}
// Dependencies: {}
impl ser :: SerializeTupleStruct for SerializeTupleStruct { type Ok = Value ; type Error = SerializerError ; fn serialize_field < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize , { let value = value . serialize (Serializer) ? ; self . 0 . push (value) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (Value :: Seq (self . 0)) } }
};
}
