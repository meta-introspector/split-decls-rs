// Generated macro for sort_by_key (function)
macro_rules! Depcrate_tests_recursivesort_by_key {
() => {
// Module: crate::tests::recursive
// Provides: {"sort_by_key"}
// Dependencies: {}
# [test] fn sort_by_key () { let dir = Dir :: tmp () ; dir . mkdirp ("foo/bar/baz/abc") ; dir . mkdirp ("quux") ; let wd = WalkDir :: new (dir . path ()) . sort_by_key (| a | a . file_name () . to_owned ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("foo") , dir . join ("foo") . join ("bar") , dir . join ("foo") . join ("bar") . join ("baz") , dir . join ("foo") . join ("bar") . join ("baz") . join ("abc") , dir . join ("quux") ,] ; assert_eq ! (expected , r . paths ()) ; }
};
}
