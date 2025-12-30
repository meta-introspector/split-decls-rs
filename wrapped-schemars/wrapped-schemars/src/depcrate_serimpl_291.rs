// Generated macro for impl_291 (impl)
macro_rules! Depcrate_serimpl_291 {
() => {
// Module: crate::ser
// Provides: {"impl_291"}
// Dependencies: {}
impl serde :: ser :: SerializeStructVariant for Serializer < '_ > { type Ok = Schema ; type Error = Error ; fn serialize_field < T > (& mut self , _key : & 'static str , _value : & T) -> Result < () , Self :: Error > where T : serde :: Serialize + ? Sized , { Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (true . into ()) } }
};
}
