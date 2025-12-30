// Generated macro for impl_57 (impl)
macro_rules! Depcrate_dataimpl_57 {
() => {
// Module: crate::data
// Provides: {"impl_57"}
// Dependencies: {}
impl Deref for Map { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const Map as * const u8 , mem :: size_of :: < Map > ()) } } }
};
}
