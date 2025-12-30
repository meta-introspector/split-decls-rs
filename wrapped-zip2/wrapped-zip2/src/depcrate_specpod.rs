// Generated macro for Pod (trait)
macro_rules! Depcrate_specPod {
() => {
// Module: crate::spec
// Provides: {"Pod"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = ""] # [doc = " - No padding/uninit bytes"] # [doc = " - All bytes patterns must be valid"] # [doc = " - No cell, pointers"] # [doc = ""] # [doc = " See `bytemuck::Pod` for more details."] pub (crate) unsafe trait Pod : Copy + 'static { # [inline] fn zeroed () -> Self { unsafe { mem :: zeroed () } } # [inline] fn as_bytes (& self) -> & [u8] { unsafe { slice :: from_raw_parts (self as * const Self as * const u8 , mem :: size_of :: < Self > ()) } } # [inline] fn as_bytes_mut (& mut self) -> & mut [u8] { unsafe { slice :: from_raw_parts_mut (self as * mut Self as * mut u8 , mem :: size_of :: < Self > ()) } } }
};
}
