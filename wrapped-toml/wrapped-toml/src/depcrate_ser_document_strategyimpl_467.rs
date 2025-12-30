// Generated macro for impl_467 (impl)
macro_rules! Depcrate_ser_document_strategyimpl_467 {
() => {
// Module: crate::ser::document::strategy
// Provides: {"impl_467"}
// Dependencies: {}
impl serde_core :: ser :: SerializeTuple for ArrayWalkValue { type Ok = core :: convert :: Infallible ; type Error = SerializationStrategy ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { self . serialize_element (value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . end () } }
};
}
