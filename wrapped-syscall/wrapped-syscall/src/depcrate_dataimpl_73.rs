// Generated macro for impl_73 (impl)
macro_rules! Depcrate_dataimpl_73 {
() => {
// Module: crate::data
// Provides: {"impl_73"}
// Dependencies: {}
impl DerefMut for PtraceEvent { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut PtraceEvent as * mut u8 , mem :: size_of :: < PtraceEvent > () ,) } } }
};
}
