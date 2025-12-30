// Generated macro for impl_97 (impl)
macro_rules! Depcrate_serimpl_97 {
() => {
// Module: crate::ser
// Provides: {"impl_97"}
// Dependencies: {}
impl < 'input , 'output , Target > ser :: SerializeSeq for SeqSerializer < 'input , 'output , Target > where Target : 'output + UrlEncodedTarget , { type Ok = & 'output mut UrlEncodedSerializer < 'input , Target > ; type Error = Error ; fn serialize_element < T : ? Sized + ser :: Serialize > (& mut self , value : & T ,) -> Result < () , Error > { value . serialize (pair :: PairSerializer :: new (self . urlencoder)) } fn end (self) -> Result < Self :: Ok , Error > { Ok (self . urlencoder) } }
};
}
