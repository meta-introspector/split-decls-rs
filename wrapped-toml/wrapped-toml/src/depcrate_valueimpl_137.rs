// Generated macro for impl_137 (impl)
macro_rules! Depcrate_valueimpl_137 {
() => {
// Module: crate::value
// Provides: {"impl_137"}
// Dependencies: {}
impl ser :: SerializeStructVariant for ValueSerializeVariant < ValueSerializeMap > { type Ok = Value ; type Error = crate :: ser :: Error ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeStruct :: serialize_field (& mut self . inner , key , value) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { let inner = ser :: SerializeStruct :: end (self . inner) ? ; let mut table = Table :: new () ; table . insert (self . variant . to_owned () , inner) ; Ok (Value :: Table (table)) } }
};
}
