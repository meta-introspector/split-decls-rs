// Generated macro for Uint (trait)
macro_rules! Depcrate_asciiUint {
() => {
// Module: crate::ascii
// Provides: {"Uint"}
// Dependencies: {}
# [doc = " Metadata for parsing unsigned integers, see [`dec_uint`]"] pub trait Uint : Sized { # [doc (hidden)] fn try_from_dec_uint (slice : & str) -> Option < Self > ; }
};
}
