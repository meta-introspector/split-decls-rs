// Generated macro for impl_73 (impl)
macro_rules! Depcrate_serializerimpl_73 {
() => {
// Module: crate::serializer
// Provides: {"impl_73"}
// Dependencies: {}
impl ser :: SerializeStructVariant for SerializeStructVariant { type Ok = ConstValue ; type Error = SerializerError ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let key = Name :: new (key) ; let value = value . serialize (Serializer) ? ; self . 1 . insert (key , value) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { let mut map = IndexMap :: new () ; map . insert (self . 0 , ConstValue :: Object (self . 1)) ; Ok (ConstValue :: Object (map)) } }
};
}
