// Generated macro for sort_by_file_name (function)
macro_rules! Depcrate_tests_recursivesort_by_file_name {
() => {
// Module: crate::tests::recursive
// Provides: {"sort_by_file_name"}
// Dependencies: {}
# [test] fn sort_by_file_name () { let dir = Dir :: tmp () ; dir . mkdirp ("foo/bar/baz/abc") ; dir . mkdirp ("quux") ; let wd = WalkDir :: new (dir . path ()) . sort_by_file_name () ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("foo") , dir . join ("foo") . join ("bar") , dir . join ("foo") . join ("bar") . join ("baz") , dir . join ("foo") . join ("bar") . join ("baz") . join ("abc") , dir . join ("quux") ,] ; assert_eq ! (expected , r . paths ()) ; }
};
}
