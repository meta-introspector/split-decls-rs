// Generated macro for ModifiedChunk (struct)
macro_rules! Depcrate_rustfmt_diffModifiedChunk {
() => {
// Module: crate::rustfmt_diff
// Provides: {"ModifiedChunk"}
// Dependencies: {}
# [doc = " A single span of changed lines, with 0 or more removed lines"] # [doc = " and a vector of 0 or more inserted lines."] # [derive (Debug , PartialEq , Eq)] pub struct ModifiedChunk { # [doc = " The first to be removed from the original text"] pub line_number_orig : u32 , # [doc = " The number of lines which have been replaced"] pub lines_removed : u32 , # [doc = " The new lines"] pub lines : Vec < String > , }
};
}
