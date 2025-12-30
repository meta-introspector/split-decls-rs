// Generated macro for impl_470 (impl)
macro_rules! Depcrate_ser_document_strategyimpl_470 {
() => {
// Module: crate::ser::document::strategy
// Provides: {"impl_470"}
// Dependencies: {}
impl serde_core :: ser :: SerializeMap for StructWalkValue { type Ok = core :: convert :: Infallible ; type Error = SerializationStrategy ; fn serialize_key < T > (& mut self , _input : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { Ok (()) } fn serialize_value < T > (& mut self , _value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Err (SerializationStrategy :: Value) } }
};
}
