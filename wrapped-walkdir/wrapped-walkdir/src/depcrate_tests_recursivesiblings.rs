// Generated macro for siblings (function)
macro_rules! Depcrate_tests_recursivesiblings {
() => {
// Module: crate::tests::recursive
// Provides: {"siblings"}
// Dependencies: {}
# [test] fn siblings () { let dir = Dir :: tmp () ; dir . mkdirp ("foo") ; dir . mkdirp ("bar") ; dir . touch_all (& ["foo/a" , "foo/b"]) ; dir . touch_all (& ["bar/a" , "bar/b"]) ; let wd = WalkDir :: new (dir . path ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("bar") , dir . join ("bar") . join ("a") , dir . join ("bar") . join ("b") , dir . join ("foo") , dir . join ("foo") . join ("a") , dir . join ("foo") . join ("b") ,] ; assert_eq ! (expected , r . sorted_paths ()) ; }
};
}
