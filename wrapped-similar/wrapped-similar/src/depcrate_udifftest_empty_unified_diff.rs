// Generated macro for test_empty_unified_diff (function)
macro_rules! Depcrate_udifftest_empty_unified_diff {
() => {
// Module: crate::udiff
// Provides: {"test_empty_unified_diff"}
// Dependencies: {}
# [test] fn test_empty_unified_diff () { let diff = TextDiff :: from_lines ("abc" , "abc") ; assert_eq ! (diff . unified_diff () . header ("a.txt" , "b.txt") . to_string () , "") ; }
};
}
