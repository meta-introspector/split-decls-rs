// Generated macro for impl_468 (impl)
macro_rules! Depcrate_ser_document_strategyimpl_468 {
() => {
// Module: crate::ser::document::strategy
// Provides: {"impl_468"}
// Dependencies: {}
impl serde_core :: ser :: SerializeTupleStruct for ArrayWalkValue { type Ok = core :: convert :: Infallible ; type Error = SerializationStrategy ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { self . serialize_element (value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . end () } }
};
}
