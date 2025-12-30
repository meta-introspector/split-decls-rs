// Generated macro for test_unified_diff (function)
macro_rules! Depcrate_texttest_unified_diff {
() => {
// Module: crate::text
// Provides: {"test_unified_diff"}
// Dependencies: {}
# [test] fn test_unified_diff () { let diff = TextDiff :: from_lines ("Hello World\nsome stuff here\nsome more stuff here\n" , "Hello World\nsome amazing stuff here\nsome more stuff here\n" ,) ; assert ! (diff . newline_terminated ()) ; insta :: assert_snapshot ! (& diff . unified_diff () . context_radius (3) . header ("old" , "new") . to_string ()) ; }
};
}
