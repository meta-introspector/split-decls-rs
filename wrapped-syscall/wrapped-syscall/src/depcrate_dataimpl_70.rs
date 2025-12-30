// Generated macro for impl_70 (impl)
macro_rules! Depcrate_dataimpl_70 {
() => {
// Module: crate::data
// Provides: {"impl_70"}
// Dependencies: {}
impl DerefMut for TimeSpec { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut TimeSpec as * mut u8 , mem :: size_of :: < TimeSpec > ()) } } }
};
}
