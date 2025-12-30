// Generated macro for impl_61 (impl)
macro_rules! Depcrate_dataimpl_61 {
() => {
// Module: crate::data
// Provides: {"impl_61"}
// Dependencies: {}
impl DerefMut for Packet { fn deref_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut Packet as * mut u8 , mem :: size_of :: < Packet > ()) } } }
};
}
