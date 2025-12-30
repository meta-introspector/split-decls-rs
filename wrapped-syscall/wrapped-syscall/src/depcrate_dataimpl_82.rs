// Generated macro for impl_82 (impl)
macro_rules! Depcrate_dataimpl_82 {
() => {
// Module: crate::data
// Provides: {"impl_82"}
// Dependencies: {}
impl DerefMut for SetSighandlerData { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut Self as * mut u8 , mem :: size_of :: < Self > ()) } } }
};
}
