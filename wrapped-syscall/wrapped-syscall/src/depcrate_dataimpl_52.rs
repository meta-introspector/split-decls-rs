// Generated macro for impl_52 (impl)
macro_rules! Depcrate_dataimpl_52 {
() => {
// Module: crate::data
// Provides: {"impl_52"}
// Dependencies: {}
impl DerefMut for ITimerSpec { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut ITimerSpec as * mut u8 , mem :: size_of :: < ITimerSpec > () ,) } } }
};
}
