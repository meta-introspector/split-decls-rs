// Generated macro for many_mixed (function)
macro_rules! Depcrate_tests_recursivemany_mixed {
() => {
// Module: crate::tests::recursive
// Provides: {"many_mixed"}
// Dependencies: {}
# [test] fn many_mixed () { let dir = Dir :: tmp () ; dir . mkdirp ("foo/a") ; dir . mkdirp ("foo/c") ; dir . mkdirp ("foo/e") ; dir . touch_all (& ["foo/b" , "foo/d" , "foo/f"]) ; let wd = WalkDir :: new (dir . path ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("foo") , dir . join ("foo") . join ("a") , dir . join ("foo") . join ("b") , dir . join ("foo") . join ("c") , dir . join ("foo") . join ("d") , dir . join ("foo") . join ("e") , dir . join ("foo") . join ("f") ,] ; assert_eq ! (expected , r . sorted_paths ()) ; }
};
}
