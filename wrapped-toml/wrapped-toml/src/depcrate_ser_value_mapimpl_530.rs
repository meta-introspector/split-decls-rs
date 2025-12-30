// Generated macro for impl_530 (impl)
macro_rules! Depcrate_ser_value_mapimpl_530 {
() => {
// Module: crate::ser::value::map
// Provides: {"impl_530"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeMap for SerializeMap < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_key < T > (& mut self , input : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { match self { Self :: Datetime (s) => s . serialize_key (input) , Self :: Table (s) => s . serialize_key (input) , } } fn serialize_value < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { match self { Self :: Datetime (s) => s . serialize_value (value) , Self :: Table (s) => s . serialize_value (value) , } } fn end (self) -> Result < Self :: Ok , Self :: Error > { match self { Self :: Datetime (s) => s . end () , Self :: Table (s) => s . end () , } } }
};
}
