// Generated macro for impl_52 (impl)
macro_rules! Depcrate_to_valueimpl_52 {
() => {
// Module: crate::to_value
// Provides: {"impl_52"}
// Dependencies: {}
impl < 'sval , S : sval :: Stream < 'sval > > serde_core :: ser :: SerializeMap for Stream < S > { type Ok = () ; type Error = Error ; fn serialize_key < T : ? Sized > (& mut self , key : & T) -> Result < () , Self :: Error > where T : serde_core :: Serialize , { self . stream . map_key_begin () . map_err (| _ | Error :: custom ("failed to stream a map key")) ? ; self . stream . value_computed (& ToValue (key)) . map_err (| _ | Error :: custom ("failed to stream a map key")) ? ; self . stream . map_key_end () . map_err (| _ | Error :: custom ("failed to stream a map key")) } fn serialize_value < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: Serialize , { self . stream . map_value_begin () . map_err (| _ | Error :: custom ("failed to stream a map value")) ? ; self . stream . value_computed (& ToValue (value)) . map_err (| _ | Error :: custom ("failed to stream a map value")) ? ; self . stream . map_value_end () . map_err (| _ | Error :: custom ("failed to stream a map value")) } fn end (mut self) -> Result < Self :: Ok , Self :: Error > { self . stream . map_end () . map_err (| _ | Error :: custom ("failed to stream a map")) } }
};
}
