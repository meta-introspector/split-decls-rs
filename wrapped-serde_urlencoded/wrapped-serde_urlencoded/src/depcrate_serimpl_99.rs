// Generated macro for impl_99 (impl)
macro_rules! Depcrate_serimpl_99 {
() => {
// Module: crate::ser
// Provides: {"impl_99"}
// Dependencies: {}
impl < 'input , 'output , Target > ser :: SerializeTupleStruct for TupleStructSerializer < 'input , 'output , Target > where Target : 'output + UrlEncodedTarget , { type Ok = & 'output mut UrlEncodedSerializer < 'input , Target > ; type Error = Error ; fn serialize_field < T : ? Sized + ser :: Serialize > (& mut self , value : & T ,) -> Result < () , Error > { self . inner . serialize_field (value) } fn end (self) -> Result < Self :: Ok , Error > { self . inner . end () } }
};
}
