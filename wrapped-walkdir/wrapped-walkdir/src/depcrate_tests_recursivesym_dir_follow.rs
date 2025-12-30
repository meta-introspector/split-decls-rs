// Generated macro for sym_dir_follow (function)
macro_rules! Depcrate_tests_recursivesym_dir_follow {
() => {
// Module: crate::tests::recursive
// Provides: {"sym_dir_follow"}
// Dependencies: {}
# [test] fn sym_dir_follow () { let dir = Dir :: tmp () ; dir . mkdirp ("a") ; dir . symlink_dir ("a" , "a-link") ; dir . touch ("a/zzz") ; let wd = WalkDir :: new (dir . path ()) . follow_links (true) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let ents = r . sorted_ents () ; assert_eq ! (5 , ents . len ()) ; let (src , link) = (& ents [1] , & ents [3]) ; assert_eq ! (dir . join ("a") , src . path ()) ; assert_eq ! (dir . join ("a-link") , link . path ()) ; assert ! (! src . path_is_symlink ()) ; assert ! (link . path_is_symlink ()) ; assert_eq ! (dir . join ("a") , fs :: read_link (link . path ()) . unwrap ()) ; assert_eq ! (1 , src . depth ()) ; assert_eq ! (1 , link . depth ()) ; assert ! (src . file_type () . is_dir ()) ; assert ! (! link . file_type () . is_symlink ()) ; assert ! (! link . file_type () . is_file ()) ; assert ! (link . file_type () . is_dir ()) ; assert ! (src . metadata () . unwrap () . is_dir ()) ; assert ! (! link . metadata () . unwrap () . file_type () . is_symlink ()) ; assert ! (! link . metadata () . unwrap () . is_file ()) ; assert ! (link . metadata () . unwrap () . is_dir ()) ; let (src_zzz , link_zzz) = (& ents [2] , & ents [4]) ; assert_eq ! (dir . join ("a") . join ("zzz") , src_zzz . path ()) ; assert_eq ! (dir . join ("a-link") . join ("zzz") , link_zzz . path ()) ; assert ! (! src_zzz . path_is_symlink ()) ; assert ! (! link_zzz . path_is_symlink ()) ; }
};
}
