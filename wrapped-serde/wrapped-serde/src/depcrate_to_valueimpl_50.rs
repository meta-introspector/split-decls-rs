// Generated macro for impl_50 (impl)
macro_rules! Depcrate_to_valueimpl_50 {
() => {
// Module: crate::to_value
// Provides: {"impl_50"}
// Dependencies: {}
impl < 'sval , S : sval :: Stream < 'sval > > serde_core :: ser :: SerializeTupleStruct for StreamTuple < S > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: Serialize , { self . stream . tuple_value_begin (None , & sval :: Index :: new (self . index)) . map_err (| _ | Error :: custom ("failed to stream a tuple value")) ? ; self . stream . value_computed (& ToValue (value)) . map_err (| _ | Error :: custom ("failed to stream a tuple value")) ? ; self . stream . tuple_value_end (None , & sval :: Index :: new (self . index)) . map_err (| _ | Error :: custom ("failed to stream a tuple value")) ? ; self . index += 1 ; Ok (()) } fn end (mut self) -> Result < Self :: Ok , Self :: Error > { self . stream . tuple_end (None , self . label . as_ref () , None) . map_err (| _ | Error :: custom ("failed to stream a tuple")) } }
};
}
