// Generated macro for impl_22 (impl)
macro_rules! Depcrate_to_serializeimpl_22 {
() => {
// Module: crate::to_serialize
// Provides: {"impl_22"}
// Dependencies: {}
impl < 'sval , TOk , TError , TNamed : serde_core :: ser :: SerializeStruct < Ok = TOk , Error = TError > , TUnnamed : serde_core :: ser :: SerializeTuple < Ok = TOk , Error = TError > , > StreamRecord < 'sval > for SerializeRecord < TNamed , TUnnamed , TError > { type Ok = Result < TOk , TError > ; fn value_computed < V : sval :: Value > (& mut self , _ : Option < sval :: Tag > , label : sval :: Label , value : V ,) -> sval_nested :: Result { match self . serializer { Ok (MaybeNamed :: Named { ref mut serializer }) => { let field = label . as_static_str () . ok_or_else (| | { sval_nested :: Error :: invalid_value ("struct field label must be static") }) ? ; match serializer . serialize_field (field , & ToSerialize :: new (value)) { Ok (()) => return Ok (()) , Err (err) => { self . serializer = Err (err) ; } } } Ok (MaybeNamed :: Unnamed { ref mut serializer }) => { match serializer . serialize_element (& ToSerialize :: new (value)) { Ok (()) => return Ok (()) , Err (err) => { self . serializer = Err (err) ; } } } Err (_) => () , } Err (sval_nested :: Error :: invalid_value ("failed to serialize tuple field" ,)) } fn end (self) -> sval_nested :: Result < Self :: Ok > { match self . serializer { Ok (MaybeNamed :: Named { serializer }) => Ok (serializer . end ()) , Ok (MaybeNamed :: Unnamed { serializer }) => Ok (serializer . end ()) , Err (e) => Ok (Err (e)) , } } }
};
}
