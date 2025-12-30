// Generated macro for impl_436 (impl)
macro_rules! Depcrate_ser_impossibleimpl_436 {
() => {
// Module: crate::ser::impossible
// Provides: {"impl_436"}
// Dependencies: {}
impl < Ok , Error > SerializeMap for Impossible < Ok , Error > where Error : ser :: Error , { type Ok = Ok ; type Error = Error ; fn serialize_key < T > (& mut self , key : & T) -> Result < () , Error > where T : ? Sized + Serialize , { let _ = key ; match self . void { } } fn serialize_value < T > (& mut self , value : & T) -> Result < () , Error > where T : ? Sized + Serialize , { let _ = value ; match self . void { } } fn end (self) -> Result < Ok , Error > { match self . void { } } }
};
}
