// Generated macro for impl_79 (impl)
macro_rules! Depcrate_dataimpl_79 {
() => {
// Module: crate::data
// Provides: {"impl_79"}
// Dependencies: {}
impl DerefMut for GrantDesc { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut GrantDesc as * mut u8 , mem :: size_of :: < GrantDesc > () ,) } } }
};
}
