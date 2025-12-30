// Generated macro for impl_535 (impl)
macro_rules! Depcrate_ser_value_mapimpl_535 {
() => {
// Module: crate::ser::value::map
// Provides: {"impl_535"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeStruct for SerializeDatetime < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { self . inner . serialize_field (key , value) . map_err (dt_err) ? ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { let value = self . inner . end () . map_err (dt_err) ? ; write ! (self . dst , "{value}") ? ; Ok (self . dst) } }
};
}
