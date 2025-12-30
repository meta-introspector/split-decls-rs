// Generated macro for impl_188 (impl)
macro_rules! Depcrate_serimpl_188 {
() => {
// Module: crate::ser
// Provides: {"impl_188"}
// Dependencies: {}
impl < E > SerializeSeq for ContentSerializeSeq < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , E > where T : ? Sized + Serialize , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . elements . push (value) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: Seq (self . elements)) } }
};
}
