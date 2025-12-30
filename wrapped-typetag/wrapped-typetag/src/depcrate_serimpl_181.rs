// Generated macro for impl_181 (impl)
macro_rules! Depcrate_serimpl_181 {
() => {
// Module: crate::ser
// Provides: {"impl_181"}
// Dependencies: {}
impl < M > SerializeStructVariant for SerializeStructVariantAsMapValue < M > where M : SerializeMap , { type Ok = M :: Ok ; type Error = M :: Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , M :: Error > where T : ? Sized + Serialize , { let value = value . serialize (ContentSerializer :: < M :: Error > :: new ()) ? ; self . fields . push ((key , value)) ; Ok (()) } fn end (mut self) -> Result < M :: Ok , M :: Error > { self . map . serialize_value (& Content :: Struct (self . name , self . fields)) ? ; self . map . end () } }
};
}
