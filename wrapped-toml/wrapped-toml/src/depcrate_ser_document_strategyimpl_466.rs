// Generated macro for impl_466 (impl)
macro_rules! Depcrate_ser_document_strategyimpl_466 {
() => {
// Module: crate::ser::document::strategy
// Provides: {"impl_466"}
// Dependencies: {}
impl serde_core :: ser :: SerializeSeq for ArrayWalkValue { type Ok = core :: convert :: Infallible ; type Error = SerializationStrategy ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { self . serialize_element (value) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . end () } }
};
}
