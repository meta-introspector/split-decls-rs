// Generated macro for impl_58 (impl)
macro_rules! Depcrate_serimpl_58 {
() => {
// Module: crate::ser
// Provides: {"impl_58"}
// Dependencies: {}
impl ser :: SerializeTupleVariant for SerializeTupleVariant { type Ok = Value ; type Error = SerializerError ; fn serialize_field < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize , { let value = value . serialize (Serializer) ? ; self . 1 . push (value) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { let mut map = BTreeMap :: new () ; map . insert (self . 0 , Value :: Seq (self . 1)) ; Ok (Value :: Map (map)) } }
};
}
