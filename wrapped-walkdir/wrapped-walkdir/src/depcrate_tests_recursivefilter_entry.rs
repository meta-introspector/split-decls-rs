// Generated macro for filter_entry (function)
macro_rules! Depcrate_tests_recursivefilter_entry {
() => {
// Module: crate::tests::recursive
// Provides: {"filter_entry"}
// Dependencies: {}
# [test] fn filter_entry () { let dir = Dir :: tmp () ; dir . mkdirp ("foo/bar/baz/abc") ; dir . mkdirp ("quux") ; let wd = WalkDir :: new (dir . path ()) . into_iter () . filter_entry (| ent | ent . file_name () != "baz") ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("foo") , dir . join ("foo") . join ("bar") , dir . join ("quux") ,] ; assert_eq ! (expected , r . sorted_paths ()) ; }
};
}
