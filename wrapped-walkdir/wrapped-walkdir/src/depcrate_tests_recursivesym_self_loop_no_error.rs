// Generated macro for sym_self_loop_no_error (function)
macro_rules! Depcrate_tests_recursivesym_self_loop_no_error {
() => {
// Module: crate::tests::recursive
// Provides: {"sym_self_loop_no_error"}
// Dependencies: {}
# [test] fn sym_self_loop_no_error () { let dir = Dir :: tmp () ; dir . symlink_file ("a" , "a") ; let wd = WalkDir :: new (dir . path ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; assert_eq ! (2 , r . ents () . len ()) ; let ent = & r . ents () [1] ; assert_eq ! (dir . join ("a") , ent . path ()) ; assert ! (ent . path_is_symlink ()) ; assert ! (ent . file_type () . is_symlink ()) ; assert ! (! ent . file_type () . is_file ()) ; assert ! (! ent . file_type () . is_dir ()) ; assert ! (ent . metadata () . unwrap () . file_type () . is_symlink ()) ; assert ! (! ent . metadata () . unwrap () . file_type () . is_file ()) ; assert ! (! ent . metadata () . unwrap () . file_type () . is_dir ()) ; }
};
}
