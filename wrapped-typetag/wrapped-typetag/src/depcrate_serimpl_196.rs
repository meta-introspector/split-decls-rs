// Generated macro for impl_196 (impl)
macro_rules! Depcrate_serimpl_196 {
() => {
// Module: crate::ser
// Provides: {"impl_196"}
// Dependencies: {}
impl < E > SerializeMap for ContentSerializeMap < E > where E : ser :: Error , { type Ok = Content ; type Error = E ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , E > where T : ? Sized + Serialize , { let key = key . serialize (ContentSerializer :: < E > :: new ()) ? ; self . key = Some (key) ; Ok (()) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , E > where T : ? Sized + Serialize , { let key = self . key . take () . expect ("serialize_value called before serialize_key") ; let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . entries . push ((key , value)) ; Ok (()) } fn end (self) -> Result < Content , E > { Ok (Content :: Map (self . entries)) } fn serialize_entry < K , V > (& mut self , key : & K , value : & V) -> Result < () , E > where K : ? Sized + Serialize , V : ? Sized + Serialize , { let key = key . serialize (ContentSerializer :: < E > :: new ()) ? ; let value = value . serialize (ContentSerializer :: < E > :: new ()) ? ; self . entries . push ((key , value)) ; Ok (()) } }
};
}
