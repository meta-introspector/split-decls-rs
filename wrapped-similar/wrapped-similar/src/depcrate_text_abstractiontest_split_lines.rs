// Generated macro for test_split_lines (function)
macro_rules! Depcrate_text_abstractiontest_split_lines {
() => {
// Module: crate::text::abstraction
// Provides: {"test_split_lines"}
// Dependencies: {}
# [test] fn test_split_lines () { assert_eq ! (DiffableStr :: tokenize_lines ("first\nsecond\rthird\r\nfourth\nlast") , vec ! ["first\n" , "second\r" , "third\r\n" , "fourth\n" , "last"]) ; assert_eq ! (DiffableStr :: tokenize_lines ("\n\n") , vec ! ["\n" , "\n"]) ; assert_eq ! (DiffableStr :: tokenize_lines ("\n") , vec ! ["\n"]) ; assert ! (DiffableStr :: tokenize_lines ("") . is_empty ()) ; }
};
}
