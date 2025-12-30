// Generated macro for impl_546 (impl)
macro_rules! Depcrate_ser_value_mapimpl_546 {
() => {
// Module: crate::ser::value::map
// Provides: {"impl_546"}
// Dependencies: {}
impl < 'd > serde_core :: ser :: SerializeStructVariant for SerializeStructVariant < 'd > { type Ok = & 'd mut String ; type Error = Error ; # [inline] fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Self :: Error > where T : serde_core :: ser :: Serialize + ? Sized , { serde_core :: ser :: SerializeStruct :: serialize_field (& mut self . inner , key , value) } # [inline] fn end (self) -> Result < Self :: Ok , Self :: Error > { let dst = serde_core :: ser :: SerializeStruct :: end (self . inner) ? ; dst . space () ? ; dst . close_inline_table () ? ; Ok (dst) } }
};
}
