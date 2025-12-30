// Generated macro for min_max_depth_diff_nada (function)
macro_rules! Depcrate_tests_recursivemin_max_depth_diff_nada {
() => {
// Module: crate::tests::recursive
// Provides: {"min_max_depth_diff_nada"}
// Dependencies: {}
# [test] fn min_max_depth_diff_nada () { let dir = Dir :: tmp () ; dir . mkdirp ("a/b/c") ; let wd = WalkDir :: new (dir . path ()) . min_depth (3) . max_depth (2) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . join ("a") . join ("b") . join ("c")] ; assert_eq ! (expected , r . sorted_paths ()) ; }
};
}
