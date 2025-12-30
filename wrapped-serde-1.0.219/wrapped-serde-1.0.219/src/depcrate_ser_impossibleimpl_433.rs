// Generated macro for impl_433 (impl)
macro_rules! Depcrate_ser_impossibleimpl_433 {
() => {
// Module: crate::ser::impossible
// Provides: {"impl_433"}
// Dependencies: {}
impl < Ok , Error > SerializeTuple for Impossible < Ok , Error > where Error : ser :: Error , { type Ok = Ok ; type Error = Error ; fn serialize_element < T > (& mut self , value : & T) -> Result < () , Error > where T : ? Sized + Serialize , { let _ = value ; match self . void { } } fn end (self) -> Result < Ok , Error > { match self . void { } } }
};
}
