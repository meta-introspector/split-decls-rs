// Generated macro for impl_35 (impl)
macro_rules! Depcrate_fmtimpl_35 {
() => {
// Module: crate::fmt
// Provides: {"impl_35"}
// Dependencies: {}
unsafe impl Slice for str { # [inline (always)] fn as_bytes (& self) -> & [u8] { str :: as_bytes (self) } # [inline (always)] unsafe fn from_bytes (x : & [u8]) -> & str { str :: from_utf8_unchecked (x) } # [inline (always)] unsafe fn from_mut_bytes (x : & mut [u8]) -> & mut str { mem :: transmute (x) } }
};
}
