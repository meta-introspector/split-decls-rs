// Generated macro for test_split_graphemes (function)
macro_rules! Depcrate_text_abstractiontest_split_graphemes {
() => {
// Module: crate::text::abstraction
// Provides: {"test_split_graphemes"}
// Dependencies: {}
# [test] # [cfg (feature = "unicode")] fn test_split_graphemes () { assert_eq ! (DiffableStr :: tokenize_graphemes ("abcfö❄️") , vec ! ["a" , "b" , "c" , "f" , "ö" , "❄️"]) ; }
};
}
