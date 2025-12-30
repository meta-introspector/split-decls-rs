// Generated macro for impl_190 (impl)
macro_rules! Depcrate_serimpl_190 {
() => {
// Module: crate::ser
// Provides: {"impl_190"}
// Dependencies: {}
impl < E > SerializeTuple for ContentSerializeTuple < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , E > where T : ? Sized + Serialize , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . elements . push (value) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: Tuple (self . elements)) } }
};
}
