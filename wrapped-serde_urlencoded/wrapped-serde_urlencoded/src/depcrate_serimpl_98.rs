// Generated macro for impl_98 (impl)
macro_rules! Depcrate_serimpl_98 {
() => {
// Module: crate::ser
// Provides: {"impl_98"}
// Dependencies: {}
impl < 'input , 'output , Target > ser :: SerializeTuple for TupleSerializer < 'input , 'output , Target > where Target : 'output + UrlEncodedTarget , { type Ok = & 'output mut UrlEncodedSerializer < 'input , Target > ; type Error = Error ; fn serialize_element < T : ? Sized + ser :: Serialize > (& mut self , value : & T ,) -> Result < () , Error > { value . serialize (pair :: PairSerializer :: new (self . urlencoder)) } fn end (self) -> Result < Self :: Ok , Error > { Ok (self . urlencoder) } }
};
}
