// Generated macro for impl_72 (impl)
macro_rules! Depcrate_dataimpl_72 {
() => {
// Module: crate::data
// Provides: {"impl_72"}
// Dependencies: {}
impl Deref for PtraceEvent { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const PtraceEvent as * const u8 , mem :: size_of :: < PtraceEvent > () ,) } } }
};
}
