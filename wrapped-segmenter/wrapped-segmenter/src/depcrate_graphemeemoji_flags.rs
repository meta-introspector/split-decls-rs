// Generated macro for emoji_flags (function)
macro_rules! Depcrate_graphemeemoji_flags {
() => {
// Module: crate::grapheme
// Provides: {"emoji_flags"}
// Dependencies: {}
# [test] fn emoji_flags () { let segmenter = GraphemeClusterSegmenter :: new () ; let breaks : Vec < usize > = segmenter . segment_str ("🇺🇸🏴󠁧󠁢󠁥󠁮󠁧󠁿") . collect () ; assert_eq ! (breaks , [0 , 8 , 36]) ; }
};
}
