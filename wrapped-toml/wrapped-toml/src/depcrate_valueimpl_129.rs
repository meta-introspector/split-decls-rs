// Generated macro for impl_129 (impl)
macro_rules! Depcrate_valueimpl_129 {
() => {
// Module: crate::value
// Provides: {"impl_129"}
// Dependencies: {}
impl ser :: SerializeMap for ValueSerializeMap { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { self . ser . serialize_key (key) } fn serialize_value < T > (& mut self , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { self . ser . serialize_value (value) } fn end (self) -> Result < Value , crate :: ser :: Error > { self . ser . end () . map (Value :: Table) } }
};
}
