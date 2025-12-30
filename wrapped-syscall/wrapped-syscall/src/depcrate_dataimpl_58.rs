// Generated macro for impl_58 (impl)
macro_rules! Depcrate_dataimpl_58 {
() => {
// Module: crate::data
// Provides: {"impl_58"}
// Dependencies: {}
impl DerefMut for Map { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut Map as * mut u8 , mem :: size_of :: < Map > ()) } } }
};
}
