// Generated macro for impl_21 (impl)
macro_rules! Depcrate_to_serializeimpl_21 {
() => {
// Module: crate::to_serialize
// Provides: {"impl_21"}
// Dependencies: {}
impl < 'sval , S : serde_core :: ser :: SerializeMap > StreamMap < 'sval > for SerializeMap < S , S :: Error > { type Ok = Result < S :: Ok , S :: Error > ; fn key_computed < V : sval :: Value > (& mut self , key : V) -> sval_nested :: Result { if let Ok (ref mut serializer) = self . serializer { match serializer . serialize_key (& ToSerialize :: new (key)) { Ok (()) => return Ok (()) , Err (err) => { self . serializer = Err (err) ; } } } Err (sval_nested :: Error :: invalid_value ("failed to serialize map key" ,)) } fn value_computed < V : sval :: Value > (& mut self , value : V) -> sval_nested :: Result { if let Ok (ref mut serializer) = self . serializer { match serializer . serialize_value (& ToSerialize :: new (value)) { Ok (()) => return Ok (()) , Err (err) => { self . serializer = Err (err) ; } } } Err (sval_nested :: Error :: invalid_value ("failed to serialize map value" ,)) } fn end (self) -> sval_nested :: Result < Self :: Ok > { match self . serializer { Ok (serializer) => Ok (serializer . end ()) , Err (err) => Ok (Err (err)) , } } }
};
}
