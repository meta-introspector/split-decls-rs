// Generated macro for impl_71 (impl)
macro_rules! Depcrate_serializerimpl_71 {
() => {
// Module: crate::serializer
// Provides: {"impl_71"}
// Dependencies: {}
impl ser :: SerializeStruct for SerializeStruct { type Ok = ConstValue ; type Error = SerializerError ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { let key = Name :: new (key) ; let value = value . serialize (Serializer) ? ; self . 0 . insert (key , value) ; Ok (()) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (ConstValue :: Object (self . 0)) } }
};
}
