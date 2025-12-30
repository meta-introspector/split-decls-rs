// Generated macro for impl_440 (impl)
macro_rules! Depcrate_schemev2impl_440 {
() => {
// Module: crate::schemev2
// Provides: {"impl_440"}
// Dependencies: {}
impl Deref for Sqe { type Target = [u8] ; fn deref (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const Sqe as * const u8 , mem :: size_of :: < Sqe > ()) } } }
};
}
