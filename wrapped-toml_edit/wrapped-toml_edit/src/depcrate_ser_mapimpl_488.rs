// Generated macro for impl_488 (impl)
macro_rules! Depcrate_ser_mapimpl_488 {
() => {
// Module: crate::ser::map
// Provides: {"impl_488"}
// Dependencies: {}
impl serde_core :: ser :: SerializeStruct for SerializeDatetime { type Ok = crate :: Datetime ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , < Self as serde_core :: ser :: SerializeStruct > :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { self . inner . serialize_field (key , value) . map_err (dt_err) ? ; Ok (()) } fn end (self) -> Result < < Self as serde_core :: ser :: SerializeStruct > :: Ok , < Self as serde_core :: ser :: SerializeStruct > :: Error > { let value = self . inner . end () . map_err (dt_err) ? ; Ok (value) } }
};
}
