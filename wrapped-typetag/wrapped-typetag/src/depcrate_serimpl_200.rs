// Generated macro for impl_200 (impl)
macro_rules! Depcrate_serimpl_200 {
() => {
// Module: crate::ser
// Provides: {"impl_200"}
// Dependencies: {}
impl < E > SerializeStructVariant for ContentSerializeStructVariant < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , E > where T : ? Sized + Serialize , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . fields . push ((key , value)) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: StructVariant (self . name , self . variant_index , self . variant , self . fields ,)) } }
};
}
