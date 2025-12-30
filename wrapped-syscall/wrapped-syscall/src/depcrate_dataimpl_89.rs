// Generated macro for impl_89 (impl)
macro_rules! Depcrate_dataimpl_89 {
() => {
// Module: crate::data
// Provides: {"impl_89"}
// Dependencies: {}
impl DerefMut for CtxtStsBuf { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut CtxtStsBuf as * mut u8 , mem :: size_of :: < CtxtStsBuf > () ,) } } }
};
}
