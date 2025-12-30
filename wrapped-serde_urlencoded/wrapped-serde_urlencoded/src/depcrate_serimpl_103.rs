// Generated macro for impl_103 (impl)
macro_rules! Depcrate_serimpl_103 {
() => {
// Module: crate::ser
// Provides: {"impl_103"}
// Dependencies: {}
impl < 'input , 'output , Target > ser :: SerializeStructVariant for StructVariantSerializer < 'input , 'output , Target > where Target : 'output + UrlEncodedTarget , { type Ok = & 'output mut UrlEncodedSerializer < 'input , Target > ; type Error = Error ; fn serialize_field < T : ? Sized + ser :: Serialize > (& mut self , key : & 'static str , value : & T ,) -> Result < () , Error > { self . inner . serialize_field (key , value) } fn end (self) -> Result < Self :: Ok , Error > { self . inner . end () } }
};
}
