// Generated macro for impl_65 (impl)
macro_rules! Depcrate_serializerimpl_65 {
() => {
// Module: crate::serializer
// Provides: {"impl_65"}
// Dependencies: {}
impl ser :: SerializeTupleStruct for SerializeTupleStruct { type Ok = ConstValue ; type Error = SerializerError ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let value = value . serialize (Serializer) ? ; self . 0 . push (value) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (ConstValue :: List (self . 0)) } }
};
}
