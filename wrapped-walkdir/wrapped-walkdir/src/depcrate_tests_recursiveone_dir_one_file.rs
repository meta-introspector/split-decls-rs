// Generated macro for one_dir_one_file (function)
macro_rules! Depcrate_tests_recursiveone_dir_one_file {
() => {
// Module: crate::tests::recursive
// Provides: {"one_dir_one_file"}
// Dependencies: {}
# [test] fn one_dir_one_file () { let dir = Dir :: tmp () ; dir . mkdirp ("foo") ; dir . touch ("foo/a") ; let wd = WalkDir :: new (dir . path ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("foo") , dir . join ("foo") . join ("a") ,] ; assert_eq ! (expected , r . sorted_paths ()) ; }
};
}
