// Generated macro for impl_81 (impl)
macro_rules! Depcrate_dataimpl_81 {
() => {
// Module: crate::data
// Provides: {"impl_81"}
// Dependencies: {}
impl Deref for SetSighandlerData { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const Self as * const u8 , mem :: size_of :: < Self > ()) } } }
};
}
