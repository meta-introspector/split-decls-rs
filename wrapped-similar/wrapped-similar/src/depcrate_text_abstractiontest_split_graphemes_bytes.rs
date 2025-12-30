// Generated macro for test_split_graphemes_bytes (function)
macro_rules! Depcrate_text_abstractiontest_split_graphemes_bytes {
() => {
// Module: crate::text::abstraction
// Provides: {"test_split_graphemes_bytes"}
// Dependencies: {}
# [test] # [cfg (all (feature = "bytes" , feature = "unicode"))] fn test_split_graphemes_bytes () { assert_eq ! (DiffableStr :: tokenize_graphemes ("abcfö❄️" . as_bytes ()) , vec ! [& b"a" [..] , & b"b" [..] , & b"c" [..] , & b"f" [..] , "ö" . as_bytes () , "❄️" . as_bytes ()]) ; }
};
}
