// Generated macro for Int (trait)
macro_rules! Depcrate_asciiInt {
() => {
// Module: crate::ascii
// Provides: {"Int"}
// Dependencies: {}
# [doc = " Metadata for parsing signed integers, see [`dec_int`]"] pub trait Int : Sized { # [doc (hidden)] fn try_from_dec_int (slice : & str) -> Option < Self > ; }
};
}
