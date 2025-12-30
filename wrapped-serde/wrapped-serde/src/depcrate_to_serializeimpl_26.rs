// Generated macro for impl_26 (impl)
macro_rules! Depcrate_to_serializeimpl_26 {
() => {
// Module: crate::to_serialize
// Provides: {"impl_26"}
// Dependencies: {}
impl < 'sval , S : serde_core :: ser :: SerializeTupleVariant > StreamTuple < 'sval > for SerializeTupleVariant < S , S :: Error > { type Ok = Result < S :: Ok , S :: Error > ; fn value_computed < V : sval :: Value > (& mut self , _ : Option < sval :: Tag > , _ : sval :: Index , value : V ,) -> sval_nested :: Result { if let Ok (ref mut serializer) = self . serializer { match serializer . serialize_field (& ToSerialize :: new (value)) { Ok (()) => return Ok (()) , Err (err) => { self . serializer = Err (err) ; } } } Err (sval_nested :: Error :: invalid_value ("failed to serialize tuple value" ,)) } fn end (self) -> sval_nested :: Result < Self :: Ok > { match self . serializer { Ok (serializer) => Ok (serializer . end ()) , Err (err) => Ok (Err (err)) , } } }
};
}
