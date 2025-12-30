// Generated macro for impl_58 (impl)
macro_rules! Depcrate_serimpl_58 {
() => {
// Module: crate::ser
// Provides: {"impl_58"}
// Dependencies: {}
impl < S > ser :: SerializeStructVariant for SerializeStructVariant < S > where S : ser :: SerializeStructVariant , { type Ok = S :: Ok ; type Error = S :: Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : ? Sized + ser :: Serialize , { self . ser . serialize_field (key , & Serialize :: new (value , self . param)) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . ser . end () } fn skip_field (& mut self , key : & 'static str) -> Result < () , Self :: Error > { self . ser . skip_field (key) } }
};
}
