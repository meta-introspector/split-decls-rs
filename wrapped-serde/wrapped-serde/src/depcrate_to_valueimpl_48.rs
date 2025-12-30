// Generated macro for impl_48 (impl)
macro_rules! Depcrate_to_valueimpl_48 {
() => {
// Module: crate::to_value
// Provides: {"impl_48"}
// Dependencies: {}
impl < 'sval , S : sval :: Stream < 'sval > > serde_core :: ser :: SerializeSeq for Stream < S > { type Ok = () ; type Error = Error ; fn serialize_element < T : ? Sized > (& mut self , value : & T) -> Result < () , Self :: Error > where T : serde_core :: Serialize , { self . stream . seq_value_begin () . map_err (| _ | Error :: custom ("failed to stream a sequence value")) ? ; self . stream . value_computed (& ToValue (value)) . map_err (| _ | Error :: custom ("failed to stream a sequence value")) ? ; self . stream . seq_value_end () . map_err (| _ | Error :: custom ("failed to stream a sequence value")) } fn end (mut self) -> Result < Self :: Ok , Self :: Error > { self . stream . seq_end () . map_err (| _ | Error :: custom ("failed to stream a sequence")) } }
};
}
