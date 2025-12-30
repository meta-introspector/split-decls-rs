// Generated macro for impl_69 (impl)
macro_rules! Depcrate_dataimpl_69 {
() => {
// Module: crate::data
// Provides: {"impl_69"}
// Dependencies: {}
impl Deref for TimeSpec { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const TimeSpec as * const u8 , mem :: size_of :: < TimeSpec > () ,) } } }
};
}
