// Generated macro for impl_168 (impl)
macro_rules! Depcrate_serimpl_168 {
() => {
// Module: crate::ser
// Provides: {"impl_168"}
// Dependencies: {}
impl < M > SerializeSeq for SerializeSeqAsMapValue < M > where M : SerializeMap , { type Ok = M :: Ok ; type Error = M :: Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , M :: Error > where T : ? Sized + Serialize , { let value = value . serialize (ContentSerializer :: < M :: Error > :: new ()) ? ; self . fields . push (value) ; Ok (()) } fn end (mut self) -> Result < M :: Ok , M :: Error > { self . map . serialize_value (& Content :: Seq (self . fields)) ? ; self . map . end () } }
};
}
