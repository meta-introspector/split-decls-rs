// Generated macro for impl_192 (impl)
macro_rules! Depcrate_serimpl_192 {
() => {
// Module: crate::ser
// Provides: {"impl_192"}
// Dependencies: {}
impl < E > SerializeTupleStruct for ContentSerializeTupleStruct < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , E > where T : ? Sized + Serialize , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . fields . push (value) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: TupleStruct (self . name , self . fields)) } }
};
}
