// Generated macro for impl_171 (impl)
macro_rules! Depcrate_serimpl_171 {
() => {
// Module: crate::ser
// Provides: {"impl_171"}
// Dependencies: {}
impl < M > SerializeTuple for SerializeTupleAsMapValue < M > where M : SerializeMap , { type Ok = M :: Ok ; type Error = M :: Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , M :: Error > where T : ? Sized + Serialize , { let value = value . serialize (ContentSerializer :: < M :: Error > :: new ()) ? ; self . fields . push (value) ; Ok (()) } fn end (mut self) -> Result < M :: Ok , M :: Error > { self . map . serialize_value (& Content :: Tuple (self . fields)) ? ; self . map . end () } }
};
}
