// Generated macro for sym_file_nofollow (function)
macro_rules! Depcrate_tests_recursivesym_file_nofollow {
() => {
// Module: crate::tests::recursive
// Provides: {"sym_file_nofollow"}
// Dependencies: {}
# [test] fn sym_file_nofollow () { let dir = Dir :: tmp () ; dir . touch ("a") ; dir . symlink_file ("a" , "a-link") ; let wd = WalkDir :: new (dir . path ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let ents = r . sorted_ents () ; assert_eq ! (3 , ents . len ()) ; let (src , link) = (& ents [1] , & ents [2]) ; assert_eq ! (dir . join ("a") , src . path ()) ; assert_eq ! (dir . join ("a-link") , link . path ()) ; assert ! (! src . path_is_symlink ()) ; assert ! (link . path_is_symlink ()) ; assert_eq ! (dir . join ("a") , fs :: read_link (link . path ()) . unwrap ()) ; assert_eq ! (1 , src . depth ()) ; assert_eq ! (1 , link . depth ()) ; assert ! (src . file_type () . is_file ()) ; assert ! (link . file_type () . is_symlink ()) ; assert ! (! link . file_type () . is_file ()) ; assert ! (! link . file_type () . is_dir ()) ; assert ! (src . metadata () . unwrap () . is_file ()) ; assert ! (link . metadata () . unwrap () . file_type () . is_symlink ()) ; assert ! (! link . metadata () . unwrap () . is_file ()) ; assert ! (! link . metadata () . unwrap () . is_dir ()) ; }
};
}
