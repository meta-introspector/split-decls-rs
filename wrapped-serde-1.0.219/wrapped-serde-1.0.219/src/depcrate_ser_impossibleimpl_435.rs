// Generated macro for impl_435 (impl)
macro_rules! Depcrate_ser_impossibleimpl_435 {
() => {
// Module: crate::ser::impossible
// Provides: {"impl_435"}
// Dependencies: {}
impl < Ok , Error > SerializeTupleVariant for Impossible < Ok , Error > where Error : ser :: Error , { type Ok = Ok ; type Error = Error ; fn serialize_field < T > (& mut self , value : & T) -> Result < () , Error > where T : ? Sized + Serialize , { let _ = value ; match self . void { } } fn end (self) -> Result < Ok , Error > { match self . void { } } }
};
}
