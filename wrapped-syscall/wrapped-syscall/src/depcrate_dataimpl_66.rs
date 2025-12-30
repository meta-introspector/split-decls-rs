// Generated macro for impl_66 (impl)
macro_rules! Depcrate_dataimpl_66 {
() => {
// Module: crate::data
// Provides: {"impl_66"}
// Dependencies: {}
impl Deref for StatVfs { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const StatVfs as * const u8 , mem :: size_of :: < StatVfs > () ,) } } }
};
}
