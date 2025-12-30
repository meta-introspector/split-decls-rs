// Generated macro for impl_64 (impl)
macro_rules! Depcrate_dataimpl_64 {
() => {
// Module: crate::data
// Provides: {"impl_64"}
// Dependencies: {}
impl DerefMut for Stat { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut Stat as * mut u8 , mem :: size_of :: < Stat > ()) } } }
};
}
