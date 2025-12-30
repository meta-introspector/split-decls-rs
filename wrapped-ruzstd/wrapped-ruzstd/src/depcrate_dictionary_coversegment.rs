// Generated macro for Segment (struct)
macro_rules! Depcrate_dictionary_coverSegment {
() => {
// Module: crate::dictionary::cover
// Provides: {"Segment"}
// Dependencies: {}
pub struct Segment { # [doc = " The actual contents of the segment."] pub raw : Vec < u8 > , # [doc = " A measure of how \"ideal\" a given segment would be to include in the dictionary"] # [doc = ""] # [doc = " Higher is better, there's no upper limit. This number is determined by"] # [doc = " estimating the number of occurances in a given epoch"] pub score : usize , }
};
}
