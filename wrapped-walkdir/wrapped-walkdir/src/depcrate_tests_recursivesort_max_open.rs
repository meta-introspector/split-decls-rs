// Generated macro for sort_max_open (function)
macro_rules! Depcrate_tests_recursivesort_max_open {
() => {
// Module: crate::tests::recursive
// Provides: {"sort_max_open"}
// Dependencies: {}
# [test] fn sort_max_open () { let dir = Dir :: tmp () ; dir . mkdirp ("foo/bar/baz/abc") ; dir . mkdirp ("quux") ; let wd = WalkDir :: new (dir . path ()) . max_open (1) . sort_by (| a , b | a . file_name () . cmp (b . file_name ()) . reverse ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("quux") , dir . join ("foo") , dir . join ("foo") . join ("bar") , dir . join ("foo") . join ("bar") . join ("baz") , dir . join ("foo") . join ("bar") . join ("baz") . join ("abc") ,] ; assert_eq ! (expected , r . paths ()) ; }
};
}
