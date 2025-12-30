// Generated macro for impl_51 (impl)
macro_rules! Depcrate_to_valueimpl_51 {
() => {
// Module: crate::to_value
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'sval , S : sval :: Stream < 'sval > > serde_core :: ser :: SerializeTupleVariant for StreamTupleVariant < S > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: Serialize , { self . stream . tuple_value_begin (None , & sval :: Index :: new (self . index)) . map_err (| _ | Error :: custom ("failed to stream a tuple variant value")) ? ; self . stream . value_computed (& ToValue (value)) . map_err (| _ | Error :: custom ("failed to stream a tuple value")) ? ; self . stream . tuple_value_end (None , & sval :: Index :: new (self . index)) . map_err (| _ | Error :: custom ("failed to stream a tuple variant value")) ? ; self . index += 1 ; Ok (()) } fn end (mut self) -> Result < Self :: Ok , Self :: Error > { self . stream . tuple_end (None , Some (& self . variant_label) , Some (& self . variant_index)) . map_err (| _ | Error :: custom ("failed to stream a tuple variant")) ? ; self . stream . enum_end (None , Some (& self . enum_label) , None) . map_err (| _ | Error :: custom ("failed to stream a tuple variant")) } }
};
}
