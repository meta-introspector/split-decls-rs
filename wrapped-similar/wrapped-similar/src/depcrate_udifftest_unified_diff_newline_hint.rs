// Generated macro for test_unified_diff_newline_hint (function)
macro_rules! Depcrate_udifftest_unified_diff_newline_hint {
() => {
// Module: crate::udiff
// Provides: {"test_unified_diff_newline_hint"}
// Dependencies: {}
# [test] fn test_unified_diff_newline_hint () { let diff = TextDiff :: from_lines ("a\n" , "b") ; insta :: assert_snapshot ! (& diff . unified_diff () . header ("a.txt" , "b.txt") . to_string ()) ; insta :: assert_snapshot ! (& diff . unified_diff () . missing_newline_hint (false) . header ("a.txt" , "b.txt") . to_string ()) ; }
};
}
