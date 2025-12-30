// Generated macro for sym_dir_self_loop_io_error (function)
macro_rules! Depcrate_tests_recursivesym_dir_self_loop_io_error {
() => {
// Module: crate::tests::recursive
// Provides: {"sym_dir_self_loop_io_error"}
// Dependencies: {}
# [test] fn sym_dir_self_loop_io_error () { let dir = Dir :: tmp () ; dir . symlink_dir ("a" , "a") ; let wd = WalkDir :: new (dir . path ()) . follow_links (true) ; let r = dir . run_recursive (wd) ; let (ents , errs) = (r . sorted_ents () , r . errs ()) ; assert_eq ! (1 , ents . len ()) ; assert_eq ! (1 , errs . len ()) ; let err = & errs [0] ; let expected = dir . join ("a") ; assert_eq ! (Some (&* expected) , err . path ()) ; assert_eq ! (1 , err . depth ()) ; assert ! (err . loop_ancestor () . is_none ()) ; assert ! (err . io_error () . is_some ()) ; }
};
}
