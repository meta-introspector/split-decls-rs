// Generated macro for empty_string (function)
macro_rules! Depcrate_graphemeempty_string {
() => {
// Module: crate::grapheme
// Provides: {"empty_string"}
// Dependencies: {}
# [test] fn empty_string () { let segmenter = GraphemeClusterSegmenter :: new () ; let breaks : Vec < usize > = segmenter . segment_str ("") . collect () ; assert_eq ! (breaks , [0]) ; }
};
}
