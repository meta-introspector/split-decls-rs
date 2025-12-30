// Generated macro for impl_43 (impl)
macro_rules! Depcrate_serimpl_43 {
() => {
// Module: crate::ser
// Provides: {"impl_43"}
// Dependencies: {}
impl < S > ser :: SerializeTuple for SerializeTuple < S > where S : ser :: SerializeTuple , { type Ok = S :: Ok ; type Error = S :: Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + ser :: Serialize , { self . ser . serialize_element (& Serialize :: new (value , self . param)) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . ser . end () } }
};
}
