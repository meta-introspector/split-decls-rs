// Generated macro for impl_51 (impl)
macro_rules! Depcrate_dataimpl_51 {
() => {
// Module: crate::data
// Provides: {"impl_51"}
// Dependencies: {}
impl Deref for ITimerSpec { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const ITimerSpec as * const u8 , mem :: size_of :: < ITimerSpec > () ,) } } }
};
}
