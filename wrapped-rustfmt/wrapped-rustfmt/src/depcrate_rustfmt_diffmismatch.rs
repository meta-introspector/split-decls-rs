// Generated macro for Mismatch (struct)
macro_rules! Depcrate_rustfmt_diffMismatch {
() => {
// Module: crate::rustfmt_diff
// Provides: {"Mismatch"}
// Dependencies: {}
# [derive (Debug , PartialEq)] pub (crate) struct Mismatch { # [doc = " The line number in the formatted version."] pub (crate) line_number : u32 , # [doc = " The line number in the original version."] pub (crate) line_number_orig : u32 , # [doc = " The set of lines (context and old/new) in the mismatch."] pub (crate) lines : Vec < DiffLine > , }
};
}
