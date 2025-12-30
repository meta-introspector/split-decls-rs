// Generated macro for empty_string (function)
macro_rules! Depcrate_wordempty_string {
() => {
// Module: crate::word
// Provides: {"empty_string"}
// Dependencies: {}
# [cfg (all (test , feature = "serde"))] # [test] fn empty_string () { let segmenter = WordSegmenter :: new_auto (WordBreakInvariantOptions :: default ()) ; let breaks : Vec < usize > = segmenter . segment_str ("") . collect () ; assert_eq ! (breaks , [0]) ; }
};
}
