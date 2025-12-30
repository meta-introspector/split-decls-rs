// Generated macro for impl_174 (impl)
macro_rules! Depcrate_serimpl_174 {
() => {
// Module: crate::ser
// Provides: {"impl_174"}
// Dependencies: {}
impl < M > SerializeTupleStruct for SerializeTupleStructAsMapValue < M > where M : SerializeMap , { type Ok = M :: Ok ; type Error = M :: Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , M :: Error > where T : ? Sized + Serialize , { let value = value . serialize (ContentSerializer :: < M :: Error > :: new ()) ? ; self . fields . push (value) ; Ok (()) } fn end (mut self) -> Result < M :: Ok , M :: Error > { self . map . serialize_value (& Content :: TupleStruct (self . name , self . fields)) ? ; self . map . end () } }
};
}
