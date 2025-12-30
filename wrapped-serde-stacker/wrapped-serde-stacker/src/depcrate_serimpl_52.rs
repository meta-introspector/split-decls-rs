// Generated macro for impl_52 (impl)
macro_rules! Depcrate_serimpl_52 {
() => {
// Module: crate::ser
// Provides: {"impl_52"}
// Dependencies: {}
impl < S > ser :: SerializeMap for SerializeMap < S > where S : ser :: SerializeMap , { type Ok = S :: Ok ; type Error = S :: Error ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , Self :: Error > where T : ? Sized + ser :: Serialize , { self . ser . serialize_key (& Serialize :: new (key , self . param)) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , Self :: Error > where T : ? Sized + ser :: Serialize , { self . ser . serialize_value (& Serialize :: new (value , self . param)) } fn end (self) -> Result < Self :: Ok , Self :: Error > { self . ser . end () } fn serialize_entry < K , V > (& mut self , key : & K , value : & V) -> Result < () , Self :: Error > where K : ? Sized + ser :: Serialize , V : ? Sized + ser :: Serialize , { self . ser . serialize_entry (& Serialize :: new (key , self . param) , & Serialize :: new (value , self . param) ,) } }
};
}
