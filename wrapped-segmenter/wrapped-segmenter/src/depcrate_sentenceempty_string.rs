// Generated macro for empty_string (function)
macro_rules! Depcrate_sentenceempty_string {
() => {
// Module: crate::sentence
// Provides: {"empty_string"}
// Dependencies: {}
# [cfg (all (test , feature = "serde"))] # [test] fn empty_string () { let segmenter = SentenceSegmenter :: new (Default :: default ()) ; let breaks : Vec < usize > = segmenter . segment_str ("") . collect () ; assert_eq ! (breaks , [0]) ; }
};
}
