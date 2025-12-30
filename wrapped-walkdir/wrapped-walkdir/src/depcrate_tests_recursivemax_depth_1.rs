// Generated macro for max_depth_1 (function)
macro_rules! Depcrate_tests_recursivemax_depth_1 {
() => {
// Module: crate::tests::recursive
// Provides: {"max_depth_1"}
// Dependencies: {}
# [test] fn max_depth_1 () { let dir = Dir :: tmp () ; dir . mkdirp ("a/b") ; let wd = WalkDir :: new (dir . path ()) . max_depth (1) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("a")] ; assert_eq ! (expected , r . sorted_paths ()) ; }
};
}
