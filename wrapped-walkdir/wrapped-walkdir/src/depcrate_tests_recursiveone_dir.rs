// Generated macro for one_dir (function)
macro_rules! Depcrate_tests_recursiveone_dir {
() => {
// Module: crate::tests::recursive
// Provides: {"one_dir"}
// Dependencies: {}
# [test] fn one_dir () { let dir = Dir :: tmp () ; dir . mkdirp ("a") ; let wd = WalkDir :: new (dir . path ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let ents = r . ents () ; assert_eq ! (2 , ents . len ()) ; let ent = & ents [1] ; assert_eq ! (dir . join ("a") , ent . path ()) ; assert_eq ! (1 , ent . depth ()) ; assert_eq ! ("a" , ent . file_name ()) ; assert ! (ent . file_type () . is_dir ()) ; }
};
}
