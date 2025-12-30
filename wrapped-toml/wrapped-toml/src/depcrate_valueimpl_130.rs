// Generated macro for impl_130 (impl)
macro_rules! Depcrate_valueimpl_130 {
() => {
// Module: crate::value
// Provides: {"impl_130"}
// Dependencies: {}
impl ser :: SerializeStruct for ValueSerializeMap { type Ok = Value ; type Error = crate :: ser :: Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeMap :: serialize_key (self , key) ? ; ser :: SerializeMap :: serialize_value (self , value) } fn end (self) -> Result < Value , crate :: ser :: Error > { ser :: SerializeMap :: end (self) } }
};
}
