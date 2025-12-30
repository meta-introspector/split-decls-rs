// Generated macro for impl_60 (impl)
macro_rules! Depcrate_serimpl_60 {
() => {
// Module: crate::ser
// Provides: {"impl_60"}
// Dependencies: {}
impl ser :: SerializeMap for SerializeMap { type Ok = Value ; type Error = SerializerError ; fn serialize_key < T : ? Sized > (& mut self , key : & T) -> Result < () , Self :: Error > where T : ser :: Serialize , { let key = key . serialize (Serializer) ? ; self . key = Some (key) ; Ok (()) } fn serialize_value < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize , { let value = value . serialize (Serializer) ? ; self . map . insert (self . key . take () . unwrap () , value) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (Value :: Map (self . map)) } }
};
}
