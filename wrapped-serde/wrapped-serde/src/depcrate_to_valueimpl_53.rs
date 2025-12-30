// Generated macro for impl_53 (impl)
macro_rules! Depcrate_to_valueimpl_53 {
() => {
// Module: crate::to_value
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'sval , S : sval :: Stream < 'sval > > serde_core :: ser :: SerializeStruct for StreamRecord < S > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized > (& mut self , key : & 'static str , value : & T ,) -> Result < () , Self :: Error > where T : serde_core :: Serialize , { self . stream . record_value_begin (None , & sval :: Label :: new (key)) . map_err (| _ | Error :: custom ("failed to stream a record value")) ? ; self . stream . value_computed (& ToValue (value)) . map_err (| _ | Error :: custom ("failed to stream a record value")) ? ; self . stream . record_value_end (None , & sval :: Label :: new (key)) . map_err (| _ | Error :: custom ("failed to stream a record value")) } fn end (mut self) -> Result < Self :: Ok , Self :: Error > { self . stream . record_end (None , self . label . as_ref () , None) . map_err (| _ | Error :: custom ("failed to stream a record")) } }
};
}
