// Generated macro for impl_78 (impl)
macro_rules! Depcrate_dataimpl_78 {
() => {
// Module: crate::data
// Provides: {"impl_78"}
// Dependencies: {}
impl Deref for GrantDesc { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const GrantDesc as * const u8 , mem :: size_of :: < GrantDesc > () ,) } } }
};
}
