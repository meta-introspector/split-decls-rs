// Generated macro for impl_411 (impl)
macro_rules! Depcrate_ser_document_arrayimpl_411 {
() => {
// Module: crate::ser::document::array
// Provides: {"impl_411"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeTupleVariant for SerializeDocumentTupleVariant < 'd > { type Ok = & 'd mut Buffer ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { let dst = self . table . body_mut () ; if self . style . multiline_array { dst . newline () ? ; write ! (dst , "    ") ? ; } else { if self . seen_value { dst . val_sep () ? ; dst . space () ? ; } } self . seen_value = true ; value . serialize (ValueSerializer :: with_style (dst , self . style)) ? ; if self . style . multiline_array { dst . val_sep () ? ; } Ok (()) } fn end (mut self) -> Result < Self :: Ok , Self :: Error > { let dst = self . table . body_mut () ; if self . style . multiline_array && self . seen_value { dst . newline () ? ; } dst . close_array () ? ; dst . newline () ? ; self . buf . push (self . table) ; Ok (self . buf) } }
};
}
