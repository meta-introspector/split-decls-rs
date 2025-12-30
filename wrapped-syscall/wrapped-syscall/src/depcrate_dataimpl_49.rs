// Generated macro for impl_49 (impl)
macro_rules! Depcrate_dataimpl_49 {
() => {
// Module: crate::data
// Provides: {"impl_49"}
// Dependencies: {}
impl DerefMut for Event { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut Event as * mut u8 , mem :: size_of :: < Event > ()) } } }
};
}
