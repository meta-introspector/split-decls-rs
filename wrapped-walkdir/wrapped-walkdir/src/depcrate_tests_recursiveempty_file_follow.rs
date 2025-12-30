// Generated macro for empty_file_follow (function)
macro_rules! Depcrate_tests_recursiveempty_file_follow {
() => {
// Module: crate::tests::recursive
// Provides: {"empty_file_follow"}
// Dependencies: {}
# [test] fn empty_file_follow () { let dir = Dir :: tmp () ; dir . touch ("a") ; let wd = WalkDir :: new (dir . path () . join ("a")) . follow_links (true) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; assert_eq ! (1 , r . ents () . len ()) ; let ent = & r . ents () [0] ; assert ! (ent . file_type () . is_file ()) ; assert ! (! ent . path_is_symlink ()) ; assert_eq ! (0 , ent . depth ()) ; assert_eq ! (dir . join ("a") , ent . path ()) ; assert_eq ! ("a" , ent . file_name ()) ; }
};
}
