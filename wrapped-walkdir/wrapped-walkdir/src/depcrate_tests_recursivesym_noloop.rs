// Generated macro for sym_noloop (function)
macro_rules! Depcrate_tests_recursivesym_noloop {
() => {
// Module: crate::tests::recursive
// Provides: {"sym_noloop"}
// Dependencies: {}
# [test] fn sym_noloop () { let dir = Dir :: tmp () ; dir . mkdirp ("a/b/c") ; dir . symlink_dir ("a" , "a/b/c/a-link") ; let wd = WalkDir :: new (dir . path ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; assert_eq ! (5 , r . ents () . len ()) ; }
};
}
