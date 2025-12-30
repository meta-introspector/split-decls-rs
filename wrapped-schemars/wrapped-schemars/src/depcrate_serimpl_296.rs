// Generated macro for impl_296 (impl)
macro_rules! Depcrate_serimpl_296 {
() => {
// Module: crate::ser
// Provides: {"impl_296"}
// Dependencies: {}
impl serde :: ser :: SerializeStruct for SerializeMap < '_ > { type Ok = Schema ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : serde :: Serialize + ? Sized , { let prop_schema = value . serialize (Serializer { generator : self . generator , include_title : false , }) ? ; self . properties . insert (key . to_string () , prop_schema . into ()) ; Ok (()) } fn end (self) -> Result < Self :: Ok , Self :: Error > { serde :: ser :: SerializeMap :: end (self) } }
};
}
