// Generated macro for Slice (trait)
macro_rules! Depcrate_fmtSlice {
() => {
// Module: crate::fmt
// Provides: {"Slice"}
// Dependencies: {}
# [doc = " Indicates a Rust slice type that is represented in memory as bytes."] pub unsafe trait Slice { # [doc = " Access the raw bytes of the slice."] fn as_bytes (& self) -> & [u8] ; # [doc = " Convert a byte slice to this kind of slice."] # [doc = ""] # [doc = " You may assume the buffer is *already validated*"] # [doc = " for `Format`."] unsafe fn from_bytes (x : & [u8]) -> & Self ; # [doc = " Convert a byte slice to this kind of slice."] # [doc = ""] # [doc = " You may assume the buffer is *already validated*"] # [doc = " for `Format`."] unsafe fn from_mut_bytes (x : & mut [u8]) -> & mut Self ; }
};
}
