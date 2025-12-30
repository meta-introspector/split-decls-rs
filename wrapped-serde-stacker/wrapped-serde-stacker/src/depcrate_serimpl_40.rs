// Generated macro for impl_40 (impl)
macro_rules! Depcrate_serimpl_40 {
() => {
// Module: crate::ser
// Provides: {"impl_40"}
// Dependencies: {}
impl < S > ser :: SerializeSeq for SerializeSeq < S > where S : ser :: SerializeSeq , { type Ok = S :: Ok ; type Error = S :: Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + ser :: Serialize , { self . ser . serialize_element (& Serialize :: new (value , self . param)) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . ser . end () } }
};
}
