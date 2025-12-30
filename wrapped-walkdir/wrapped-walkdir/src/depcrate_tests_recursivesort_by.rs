// Generated macro for sort_by (function)
macro_rules! Depcrate_tests_recursivesort_by {
() => {
// Module: crate::tests::recursive
// Provides: {"sort_by"}
// Dependencies: {}
# [test] fn sort_by () { let dir = Dir :: tmp () ; dir . mkdirp ("foo/bar/baz/abc") ; dir . mkdirp ("quux") ; let wd = WalkDir :: new (dir . path ()) . sort_by (| a , b | a . file_name () . cmp (b . file_name ()) . reverse ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("quux") , dir . join ("foo") , dir . join ("foo") . join ("bar") , dir . join ("foo") . join ("bar") . join ("baz") , dir . join ("foo") . join ("bar") . join ("baz") . join ("abc") ,] ; assert_eq ! (expected , r . paths ()) ; }
};
}
