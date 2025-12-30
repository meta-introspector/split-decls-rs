// Generated macro for empty (function)
macro_rules! Depcrate_tests_recursiveempty {
() => {
// Module: crate::tests::recursive
// Provides: {"empty"}
// Dependencies: {}
# [test] fn empty () { let dir = Dir :: tmp () ; let wd = WalkDir :: new (dir . path ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; assert_eq ! (1 , r . ents () . len ()) ; let ent = & r . ents () [0] ; assert ! (ent . file_type () . is_dir ()) ; assert ! (! ent . path_is_symlink ()) ; assert_eq ! (0 , ent . depth ()) ; assert_eq ! (dir . path () , ent . path ()) ; assert_eq ! (dir . path () . file_name () . unwrap () , ent . file_name ()) ; }
};
}
