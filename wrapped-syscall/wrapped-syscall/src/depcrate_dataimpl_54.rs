// Generated macro for impl_54 (impl)
macro_rules! Depcrate_dataimpl_54 {
() => {
// Module: crate::data
// Provides: {"impl_54"}
// Dependencies: {}
impl Deref for OldMap { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const OldMap as * const u8 , mem :: size_of :: < OldMap > ()) } } }
};
}
