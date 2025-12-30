// Generated macro for min_max_depth_diff_0 (function)
macro_rules! Depcrate_tests_recursivemin_max_depth_diff_0 {
() => {
// Module: crate::tests::recursive
// Provides: {"min_max_depth_diff_0"}
// Dependencies: {}
# [test] fn min_max_depth_diff_0 () { let dir = Dir :: tmp () ; dir . mkdirp ("a/b/c") ; let wd = WalkDir :: new (dir . path ()) . min_depth (2) . max_depth (2) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . join ("a") . join ("b")] ; assert_eq ! (expected , r . sorted_paths ()) ; }
};
}
