// Generated macro for impl_46 (impl)
macro_rules! Depcrate_serimpl_46 {
() => {
// Module: crate::ser
// Provides: {"impl_46"}
// Dependencies: {}
impl < S > ser :: SerializeTupleStruct for SerializeTupleStruct < S > where S : ser :: SerializeTupleStruct , { type Ok = S :: Ok ; type Error = S :: Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + ser :: Serialize , { self . ser . serialize_field (& Serialize :: new (value , self . param)) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . ser . end () } }
};
}
