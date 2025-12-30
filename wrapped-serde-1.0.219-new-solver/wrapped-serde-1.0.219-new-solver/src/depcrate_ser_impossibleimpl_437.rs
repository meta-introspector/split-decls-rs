// Generated macro for impl_437 (impl)
macro_rules! Depcrate_ser_impossibleimpl_437 {
() => {
// Module: crate::ser::impossible
// Provides: {"impl_437"}
// Dependencies: {}
impl < Ok , Error > SerializeStruct for Impossible < Ok , Error > where Error : ser :: Error , { type Ok = Ok ; type Error = Error ; fn serialize_field < T > (& mut self , key : & 'static str , value : & T) -> Result < () , Error > where T : ? Sized + Serialize , { let _ = key ; let _ = value ; match self . void { } } fn end (self) -> Result < Ok , Error > { match self . void { } } }
};
}
