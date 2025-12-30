// Generated macro for impl_471 (impl)
macro_rules! Depcrate_ser_document_strategyimpl_471 {
() => {
// Module: crate::ser::document::strategy
// Provides: {"impl_471"}
// Dependencies: {}
impl serde_core :: ser :: SerializeStruct for StructWalkValue { type Ok = core :: convert :: Infallible ; type Error = SerializationStrategy ; fn serialize_field < T > (& mut self , _key : & 'static str , _value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { Err (SerializationStrategy :: Value) } }
};
}
