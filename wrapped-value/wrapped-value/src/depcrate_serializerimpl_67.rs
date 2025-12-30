// Generated macro for impl_67 (impl)
macro_rules! Depcrate_serializerimpl_67 {
() => {
// Module: crate::serializer
// Provides: {"impl_67"}
// Dependencies: {}
impl ser :: SerializeTupleVariant for SerializeTupleVariant { type Ok = ConstValue ; type Error = SerializerError ; # [inline] fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let value = value . serialize (Serializer) ? ; self . 1 . push (value) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { let mut map = IndexMap :: new () ; map . insert (self . 0 , ConstValue :: List (self . 1)) ; Ok (ConstValue :: Object (map)) } }
};
}
