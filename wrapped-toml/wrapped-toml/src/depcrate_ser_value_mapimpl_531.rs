// Generated macro for impl_531 (impl)
macro_rules! Depcrate_ser_value_mapimpl_531 {
() => {
// Module: crate::ser::value::map
// Provides: {"impl_531"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeStruct for SerializeMap < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { match self { Self :: Datetime (s) => s . serialize_field (key , value) , Self :: Table (s) => s . serialize_field (key , value) , } } fn end (self) -> Result < Self :: Ok , Self :: Error > { match self { Self :: Datetime (s) => s . end () , Self :: Table (s) => s . end () , } } }
};
}
