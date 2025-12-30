// Generated macro for impl_506 (impl)
macro_rules! Depcrate_ser_value_arrayimpl_506 {
() => {
// Module: crate::ser::value::array
// Provides: {"impl_506"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeSeq for SerializeValueArray < 'd > { type Ok = & 'd mut String ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Error > where T : serde_core :: ser :: Serialize + ? Sized , { if self . multiline_array () { self . dst . newline () ? ; write ! (self . dst , "    ") ? ; } else { if self . seen_value { self . dst . val_sep () ? ; self . dst . space () ? ; } } self . seen_value = true ; value . serialize (super :: ValueSerializer :: with_style (self . dst , self . style)) ? ; if self . multiline_array () { self . dst . val_sep () ? ; } Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . end () } }
};
}
