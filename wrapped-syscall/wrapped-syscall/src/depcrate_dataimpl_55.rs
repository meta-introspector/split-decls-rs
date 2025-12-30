// Generated macro for impl_55 (impl)
macro_rules! Depcrate_dataimpl_55 {
() => {
// Module: crate::data
// Provides: {"impl_55"}
// Dependencies: {}
impl DerefMut for OldMap { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut OldMap as * mut u8 , mem :: size_of :: < OldMap > ()) } } }
};
}
