// Generated macro for impl_55 (impl)
macro_rules! Depcrate_serimpl_55 {
() => {
// Module: crate::ser
// Provides: {"impl_55"}
// Dependencies: {}
impl < S > ser :: SerializeStruct for SerializeStruct < S > where S : ser :: SerializeStruct , { type Ok = S :: Ok ; type Error = S :: Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ? Sized + ser :: Serialize , { self . ser . serialize_field (key , & Serialize :: new (value , self . param)) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . ser . end () } fn skip_field (& mut self , key : & 'static str) -> Result < () , Self :: Error > { self . ser . skip_field (key) } }
};
}
