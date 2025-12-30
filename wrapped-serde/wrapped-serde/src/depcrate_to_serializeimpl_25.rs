// Generated macro for impl_25 (impl)
macro_rules! Depcrate_to_serializeimpl_25 {
() => {
// Module: crate::to_serialize
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'sval , S : serde_core :: ser :: SerializeStructVariant > StreamRecord < 'sval > for SerializeRecordVariant < S , S :: Error > { type Ok = Result < S :: Ok , S :: Error > ; fn value_computed < V : sval :: Value > (& mut self , _ : Option < sval :: Tag > , label : sval :: Label , value : V ,) -> sval_nested :: Result { let field = label . as_static_str () . ok_or_else (| | { sval_nested :: Error :: invalid_value ("struct variant field label must be static") }) ? ; if let Ok (ref mut serializer) = self . serializer { match serializer . serialize_field (field , & ToSerialize :: new (value)) { Ok (()) => return Ok (()) , Err (err) => { self . serializer = Err (err) ; } } } Err (sval_nested :: Error :: invalid_value ("failed to serialize struct value" ,)) } fn end (self) -> sval_nested :: Result < Self :: Ok > { match self . serializer { Ok (serializer) => Ok (serializer . end ()) , Err (err) => Ok (Err (err)) , } } }
};
}
