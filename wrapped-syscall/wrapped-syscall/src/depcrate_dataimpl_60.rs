// Generated macro for impl_60 (impl)
macro_rules! Depcrate_dataimpl_60 {
() => {
// Module: crate::data
// Provides: {"impl_60"}
// Dependencies: {}
impl Deref for Packet { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const Packet as * const u8 , mem :: size_of :: < Packet > ()) } } }
};
}
