// Generated macro for Sequence (struct)
macro_rules! Depcrate_blocks_sequence_sectionSequence {
() => {
// Module: crate::blocks::sequence_section
// Provides: {"Sequence"}
// Dependencies: {}
# [doc = " A sequence represents potentially redundant data, and it can be broken up into 2 steps:"] # [doc = " - A copy step, where data is copied from the literals section to the decompressed output"] # [doc = " - A *match* copy step that copies data from within the previously decompressed output."] # [doc = ""] # [doc = " <https://github.com/facebook/zstd/blob/dev/doc/zstd_compression_format.md#sequence-execution>"] # [derive (Clone , Copy)] pub struct Sequence { # [doc = " Literal length, or the number of bytes to be copied from the literals section"] # [doc = " in the copy step."] pub ll : u32 , # [doc = " The length of the match to make during the match copy step."] pub ml : u32 , # [doc = " How far back to go in the decompressed data to read from the match copy step."] # [doc = " If this value is greater than 3, then the offset is `of -3`. If `of` is from 1-3,"] # [doc = " then it has special handling:"] # [doc = ""] # [doc = " The first 3 values define 3 different repeated offsets, with 1 referring to the most"] # [doc = " recent, 2 the second recent, and so on. When the current sequence has a literal length of 0,"] # [doc = " then the repeated offsets are shifted by 1. So an offset value of 1 refers to 2, 2 refers to 3,"] # [doc = " and 3 refers to the most recent offset minus one. If that value is equal to zero, the data"] # [doc = " is considered corrupted."] pub of : u32 , }
};
}
