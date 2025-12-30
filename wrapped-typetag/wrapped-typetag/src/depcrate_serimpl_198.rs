// Generated macro for impl_198 (impl)
macro_rules! Depcrate_serimpl_198 {
() => {
// Module: crate::ser
// Provides: {"impl_198"}
// Dependencies: {}
impl < E > SerializeStruct for ContentSerializeStruct < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , E > where T : ? Sized + Serialize , { let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . fields . push ((key , value)) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: Struct (self . name , self . fields)) } }
};
}
