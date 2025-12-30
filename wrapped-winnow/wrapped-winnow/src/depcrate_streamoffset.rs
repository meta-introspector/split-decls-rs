// Generated macro for Offset (trait)
macro_rules! Depcrate_streamOffset {
() => {
// Module: crate::stream
// Provides: {"Offset"}
// Dependencies: {}
# [doc = " Useful functions to calculate the offset between slices and show a hexdump of a slice"] pub trait Offset < Start = Self > { # [doc = " Offset between the first byte of `start` and the first byte of `self`a"] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **Note:** This is an offset, not an index, and may point to the end of input"] # [doc = " (`start.len()`) when `self` is exhausted."] # [doc = ""] # [doc = " </div>"] fn offset_from (& self , start : & Start) -> usize ; }
};
}
