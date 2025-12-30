// Generated macro for impl_178 (impl)
macro_rules! Depcrate_serimpl_178 {
() => {
// Module: crate::ser
// Provides: {"impl_178"}
// Dependencies: {}
impl < M > SerializeStruct for SerializeStructAsMap < M > where M : SerializeMap , { type Ok = M :: Ok ; type Error = M :: Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ? Sized + Serialize , { if key == self . tag { match expect_str (value , self . variant) { Ok (()) => Ok (()) , Err (unexpected) => Err (ser :: Error :: custom (format ! ("mismatched value for tag {:?}: {:?} vs {:?}" , self . tag , self . variant , unexpected ,))) , } } else { self . map . serialize_entry (key , value) } } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . map . end () } }
};
}
