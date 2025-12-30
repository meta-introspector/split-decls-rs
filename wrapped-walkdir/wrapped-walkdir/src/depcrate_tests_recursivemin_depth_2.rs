// Generated macro for min_depth_2 (function)
macro_rules! Depcrate_tests_recursivemin_depth_2 {
() => {
// Module: crate::tests::recursive
// Provides: {"min_depth_2"}
// Dependencies: {}
# [test] fn min_depth_2 () { let dir = Dir :: tmp () ; dir . mkdirp ("a/b") ; let wd = WalkDir :: new (dir . path ()) . min_depth (2) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . join ("a") . join ("b")] ; assert_eq ! (expected , r . sorted_paths ()) ; }
};
}
