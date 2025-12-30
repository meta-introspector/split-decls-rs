// Generated macro for impl_20 (impl)
macro_rules! Depcrate_to_serializeimpl_20 {
() => {
// Module: crate::to_serialize
// Provides: {"impl_20"}
// Dependencies: {}
impl < 'sval , S : serde_core :: ser :: SerializeSeq > StreamSeq < 'sval > for SerializeSeq < S , S :: Error > { type Ok = Result < S :: Ok , S :: Error > ; fn value_computed < V : sval :: Value > (& mut self , value : V) -> sval_nested :: Result { if let Ok (ref mut serializer) = self . serializer { match serializer . serialize_element (& ToSerialize :: new (value)) { Ok (()) => return Ok (()) , Err (err) => { self . serializer = Err (err) ; } } } Err (sval_nested :: Error :: invalid_value ("failed to serialize sequence element" ,)) } fn end (self) -> sval_nested :: Result < Self :: Ok > { match self . serializer { Ok (serializer) => Ok (serializer . end ()) , Err (err) => Ok (Err (err)) , } } }
};
}
