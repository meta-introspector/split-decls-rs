// Generated macro for impl_49 (impl)
macro_rules! Depcrate_serimpl_49 {
() => {
// Module: crate::ser
// Provides: {"impl_49"}
// Dependencies: {}
impl < S > ser :: SerializeTupleVariant for SerializeTupleVariant < S > where S : ser :: SerializeTupleVariant , { type Ok = S :: Ok ; type Error = S :: Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + ser :: Serialize , { self . ser . serialize_field (& Serialize :: new (value , self . param)) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . ser . end () } }
};
}
