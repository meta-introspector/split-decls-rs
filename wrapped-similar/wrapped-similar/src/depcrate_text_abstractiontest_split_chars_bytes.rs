// Generated macro for test_split_chars_bytes (function)
macro_rules! Depcrate_text_abstractiontest_split_chars_bytes {
() => {
// Module: crate::text::abstraction
// Provides: {"test_split_chars_bytes"}
// Dependencies: {}
# [test] # [cfg (feature = "bytes")] fn test_split_chars_bytes () { assert_eq ! (DiffableStr :: tokenize_chars ("abcfö❄️" . as_bytes ()) , vec ! [& b"a" [..] , & b"b" [..] , & b"c" [..] , & b"f" [..] , "ö" . as_bytes () , "❄" . as_bytes () , "\u{fe0f}" . as_bytes ()]) ; }
};
}
