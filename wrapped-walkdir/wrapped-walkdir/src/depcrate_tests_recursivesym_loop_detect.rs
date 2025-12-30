// Generated macro for sym_loop_detect (function)
macro_rules! Depcrate_tests_recursivesym_loop_detect {
() => {
// Module: crate::tests::recursive
// Provides: {"sym_loop_detect"}
// Dependencies: {}
# [test] fn sym_loop_detect () { let dir = Dir :: tmp () ; dir . mkdirp ("a/b/c") ; dir . symlink_dir ("a" , "a/b/c/a-link") ; let wd = WalkDir :: new (dir . path ()) . follow_links (true) ; let r = dir . run_recursive (wd) ; let (ents , errs) = (r . sorted_ents () , r . errs ()) ; assert_eq ! (4 , ents . len ()) ; assert_eq ! (1 , errs . len ()) ; let err = & errs [0] ; let expected = dir . join ("a/b/c/a-link") ; assert_eq ! (Some (&* expected) , err . path ()) ; let expected = dir . join ("a") ; assert_eq ! (Some (&* expected) , err . loop_ancestor ()) ; assert_eq ! (4 , err . depth ()) ; assert ! (err . io_error () . is_none ()) ; }
};
}
