// Generated macro for impl_102 (impl)
macro_rules! Depcrate_serimpl_102 {
() => {
// Module: crate::ser
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'input , 'output , Target > ser :: SerializeStruct for StructSerializer < 'input , 'output , Target > where Target : 'output + UrlEncodedTarget , { type Ok = & 'output mut UrlEncodedSerializer < 'input , Target > ; type Error = Error ; fn serialize_field < T : ? Sized + ser :: Serialize > (& mut self , key : & 'static str , value : & T ,) -> Result < () , Error > { let value_sink = value :: ValueSink :: new (self . urlencoder , key) ; value . serialize (part :: PartSerializer :: new (value_sink)) } fn end (self) -> Result < Self :: Ok , Error > { Ok (self . urlencoder) } }
};
}
