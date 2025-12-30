// Generated macro for max_depth_0 (function)
macro_rules! Depcrate_tests_recursivemax_depth_0 {
() => {
// Module: crate::tests::recursive
// Provides: {"max_depth_0"}
// Dependencies: {}
# [test] fn max_depth_0 () { let dir = Dir :: tmp () ; dir . mkdirp ("a/b") ; let wd = WalkDir :: new (dir . path ()) . max_depth (0) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf ()] ; assert_eq ! (expected , r . sorted_paths ()) ; }
};
}
