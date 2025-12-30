// Generated macro for impl_194 (impl)
macro_rules! Depcrate_serimpl_194 {
() => {
// Module: crate::ser
// Provides: {"impl_194"}
// Dependencies: {}
impl < E > SerializeTupleVariant for ContentSerializeTupleVariant < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , E > where T : ? Sized + Serialize , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . fields . push (value) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: TupleVariant (self . name , self . variant_index , self . variant , self . fields ,)) } }
};
}
