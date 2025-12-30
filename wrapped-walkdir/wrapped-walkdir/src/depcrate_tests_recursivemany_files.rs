// Generated macro for many_files (function)
macro_rules! Depcrate_tests_recursivemany_files {
() => {
// Module: crate::tests::recursive
// Provides: {"many_files"}
// Dependencies: {}
# [test] fn many_files () { let dir = Dir :: tmp () ; dir . mkdirp ("foo") ; dir . touch_all (& ["foo/a" , "foo/b" , "foo/c"]) ; let wd = WalkDir :: new (dir . path ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("foo") , dir . join ("foo") . join ("a") , dir . join ("foo") . join ("b") , dir . join ("foo") . join ("c") ,] ; assert_eq ! (expected , r . sorted_paths ()) ; }
};
}
