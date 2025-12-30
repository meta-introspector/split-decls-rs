// Generated macro for impl_54 (impl)
macro_rules! Depcrate_to_valueimpl_54 {
() => {
// Module: crate::to_value
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'sval , S : sval :: Stream < 'sval > > serde_core :: ser :: SerializeStructVariant for StreamRecordVariant < S > { type Ok = () ; type Error = Error ; fn serialize_field < T : ? Sized > (& mut self , key : & 'static str , value : & T ,) -> Result < () , Self :: Error > where T : serde_core :: Serialize , { self . stream . record_value_begin (None , & sval :: Label :: new (key)) . map_err (| _ | Error :: custom ("failed to stream a record variant value")) ? ; self . stream . value_computed (& ToValue (value)) . map_err (| _ | Error :: custom ("failed to stream a record variant value")) ? ; self . stream . record_value_end (None , & sval :: Label :: new (key)) . map_err (| _ | Error :: custom ("failed to stream a record variant value")) } fn end (mut self) -> Result < Self :: Ok , Self :: Error > { self . stream . record_end (None , Some (& self . variant_label) , Some (& self . variant_index)) . map_err (| _ | Error :: custom ("failed to stream a record variant")) ? ; self . stream . enum_end (None , Some (& self . enum_label) , None) . map_err (| _ | Error :: custom ("failed to stream a record variant")) } }
};
}
