// Generated macro for impl_316 (impl)
macro_rules! Depcrate_flagimpl_316 {
() => {
// Module: crate::flag
// Provides: {"impl_316"}
// Dependencies: {}
impl Deref for PtraceFlags { type Target = [u8] ; fn deref (& self) -> & Self :: Target { unsafe { slice :: from_raw_parts (& self . bits () as * const _ as * const u8 , mem :: size_of :: < u64 > ()) } } }
};
}
