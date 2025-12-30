// Generated macro for impl_465 (impl)
macro_rules! Depcrate_ser_document_strategyimpl_465 {
() => {
// Module: crate::ser::document::strategy
// Provides: {"impl_465"}
// Dependencies: {}
impl ArrayWalkValue { fn new () -> Self { Self { is_empty : true } } fn serialize_element < T > (& mut self , value : & T) -> Result < () , SerializationStrategy > where T : serde_core :: ser :: Serialize + ? Sized , { self . is_empty = false ; match SerializationStrategy :: from (value) { SerializationStrategy :: Value | SerializationStrategy :: ArrayOfTables | SerializationStrategy :: Unknown | SerializationStrategy :: Skip => Err (SerializationStrategy :: Value) , SerializationStrategy :: Table => Ok (()) , } } fn end (self) -> Result < core :: convert :: Infallible , SerializationStrategy > { if self . is_empty { Err (SerializationStrategy :: Value) } else { Err (SerializationStrategy :: ArrayOfTables) } } }
};
}
