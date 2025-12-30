// Generated macro for impl_290 (impl)
macro_rules! Depcrate_serimpl_290 {
() => {
// Module: crate::ser
// Provides: {"impl_290"}
// Dependencies: {}
impl serde :: ser :: SerializeTupleVariant for Serializer < '_ > { type Ok = Schema ; type Error = Error ; fn serialize_field < T > (& mut self , _value : & T) -> Result < () , Self :: Error > where T : serde :: Serialize + ? Sized , { Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Ok (true . into ()) } }
};
}
