// Generated macro for contents_first (function)
macro_rules! Depcrate_tests_recursivecontents_first {
() => {
// Module: crate::tests::recursive
// Provides: {"contents_first"}
// Dependencies: {}
# [test] fn contents_first () { let dir = Dir :: tmp () ; dir . touch ("a") ; let wd = WalkDir :: new (dir . path ()) . contents_first (true) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . join ("a") , dir . path () . to_path_buf ()] ; assert_eq ! (expected , r . paths ()) ; }
};
}
