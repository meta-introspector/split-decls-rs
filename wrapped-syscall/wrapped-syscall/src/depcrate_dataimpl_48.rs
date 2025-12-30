// Generated macro for impl_48 (impl)
macro_rules! Depcrate_dataimpl_48 {
() => {
// Module: crate::data
// Provides: {"impl_48"}
// Dependencies: {}
impl Deref for Event { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const Event as * const u8 , mem :: size_of :: < Event > ()) } } }
};
}
