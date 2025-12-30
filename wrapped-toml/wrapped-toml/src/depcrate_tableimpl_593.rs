// Generated macro for impl_593 (impl)
macro_rules! Depcrate_tableimpl_593 {
() => {
// Module: crate::table
// Provides: {"impl_593"}
// Dependencies: {}
impl ser :: SerializeStruct for SerializeMap { type Ok = Table ; type Error = crate :: ser :: Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , crate :: ser :: Error > where T : ser :: Serialize + ? Sized , { ser :: SerializeMap :: serialize_key (self , key) ? ; ser :: SerializeMap :: serialize_value (self , value) } fn end (self) -> Result < Table , crate :: ser :: Error > { ser :: SerializeMap :: end (self) } }
};
}
