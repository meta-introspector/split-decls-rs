// Generated macro for impl_63 (impl)
macro_rules! Depcrate_serializerimpl_63 {
() => {
// Module: crate::serializer
// Provides: {"impl_63"}
// Dependencies: {}
impl ser :: SerializeTuple for SerializeTuple { type Ok = ConstValue ; type Error = SerializerError ; # [inline] fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let value = value . serialize (Serializer) ? ; self . 0 . push (value) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (ConstValue :: List (self . 0)) } }
};
}
