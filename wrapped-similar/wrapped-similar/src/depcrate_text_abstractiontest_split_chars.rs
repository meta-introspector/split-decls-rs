// Generated macro for test_split_chars (function)
macro_rules! Depcrate_text_abstractiontest_split_chars {
() => {
// Module: crate::text::abstraction
// Provides: {"test_split_chars"}
// Dependencies: {}
# [test] fn test_split_chars () { assert_eq ! (DiffableStr :: tokenize_chars ("abcfö❄️") , vec ! ["a" , "b" , "c" , "f" , "ö" , "❄" , "\u{fe0f}"]) ; }
};
}
