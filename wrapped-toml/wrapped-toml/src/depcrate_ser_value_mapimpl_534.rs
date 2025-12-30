// Generated macro for impl_534 (impl)
macro_rules! Depcrate_ser_value_mapimpl_534 {
() => {
// Module: crate::ser::value::map
// Provides: {"impl_534"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeMap for SerializeDatetime < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_key < T > (& mut self , _input : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { unreachable ! ("datetimes should only be serialized as structs, not maps") } fn serialize_value < T > (& mut self , _value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { unreachable ! ("datetimes should only be serialized as structs, not maps") } fn end (self) -> Result < Self :: Ok , Self :: Error > { unreachable ! ("datetimes should only be serialized as structs, not maps") } }
};
}
