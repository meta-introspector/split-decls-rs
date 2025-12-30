// Generated macro for test_split_lines_bytes (function)
macro_rules! Depcrate_text_abstractiontest_split_lines_bytes {
() => {
// Module: crate::text::abstraction
// Provides: {"test_split_lines_bytes"}
// Dependencies: {}
# [test] # [cfg (feature = "bytes")] fn test_split_lines_bytes () { assert_eq ! (DiffableStr :: tokenize_lines ("first\nsecond\rthird\r\nfourth\nlast" . as_bytes ()) , vec ! ["first\n" . as_bytes () , "second\r" . as_bytes () , "third\r\n" . as_bytes () , "fourth\n" . as_bytes () , "last" . as_bytes ()]) ; assert_eq ! (DiffableStr :: tokenize_lines ("\n\n" . as_bytes ()) , vec ! ["\n" . as_bytes () , "\n" . as_bytes ()]) ; assert_eq ! (DiffableStr :: tokenize_lines ("\n" . as_bytes ()) , vec ! ["\n" . as_bytes ()]) ; assert ! (DiffableStr :: tokenize_lines ("" . as_bytes ()) . is_empty ()) ; }
};
}
