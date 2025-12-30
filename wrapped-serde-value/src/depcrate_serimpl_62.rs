// Generated macro for impl_62 (impl)
macro_rules! Depcrate_serimpl_62 {
() => {
// Module: crate::ser
// Provides: {"impl_62"}
// Dependencies: {}
impl ser :: SerializeStruct for SerializeStruct { type Ok = Value ; type Error = SerializerError ; fn serialize_field < T : ? Sized > (& mut self , key : & 'static str , value : & T ,) -> Result < () , Self :: Error > where T : ser :: Serialize , { let key = Value :: String (key . to_string ()) ; let value = value . serialize (Serializer) ? ; self . 0 . insert (key , value) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (Value :: Map (self . 0)) } }
};
}
