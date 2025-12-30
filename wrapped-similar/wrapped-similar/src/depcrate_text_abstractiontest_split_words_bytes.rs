// Generated macro for test_split_words_bytes (function)
macro_rules! Depcrate_text_abstractiontest_split_words_bytes {
() => {
// Module: crate::text::abstraction
// Provides: {"test_split_words_bytes"}
// Dependencies: {}
# [test] # [cfg (feature = "bytes")] fn test_split_words_bytes () { assert_eq ! (DiffableStr :: tokenize_words ("foo    bar baz\n\n  aha" . as_bytes ()) , [& b"foo" [..] , & b"    " [..] , & b"bar" [..] , & b" " [..] , & b"baz" [..] , & b"\n\n  " [..] , & b"aha" [..]]) ; }
};
}
