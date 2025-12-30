// Generated macro for impl_175 (impl)
macro_rules! Depcrate_serimpl_175 {
() => {
// Module: crate::ser
// Provides: {"impl_175"}
// Dependencies: {}
impl < M > SerializeTupleVariant for SerializeTupleStructAsMapValue < M > where M : SerializeMap , { type Ok = M :: Ok ; type Error = M :: Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , M :: Error > where T : ? Sized + Serialize , { let value = value . serialize (ContentSerializer :: < M :: Error > :: new ()) ? ; self . fields . push (value) ; Ok (()) } fn end (mut self) -> Result < M :: Ok , M :: Error > { self . map . serialize_value (& Content :: TupleStruct (self . name , self . fields)) ? ; self . map . end () } }
};
}
