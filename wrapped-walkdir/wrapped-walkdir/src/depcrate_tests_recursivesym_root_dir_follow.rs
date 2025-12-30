// Generated macro for sym_root_dir_follow (function)
macro_rules! Depcrate_tests_recursivesym_root_dir_follow {
() => {
// Module: crate::tests::recursive
// Provides: {"sym_root_dir_follow"}
// Dependencies: {}
# [test] fn sym_root_dir_follow () { let dir = Dir :: tmp () ; dir . mkdirp ("a") ; dir . symlink_dir ("a" , "a-link") ; dir . touch ("a/zzz") ; let wd = WalkDir :: new (dir . join ("a-link")) . follow_links (true) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let ents = r . sorted_ents () ; assert_eq ! (2 , ents . len ()) ; let link = & ents [0] ; assert_eq ! (dir . join ("a-link") , link . path ()) ; assert ! (link . path_is_symlink ()) ; assert_eq ! (dir . join ("a") , fs :: read_link (link . path ()) . unwrap ()) ; assert_eq ! (0 , link . depth ()) ; assert ! (! link . file_type () . is_symlink ()) ; assert ! (! link . file_type () . is_file ()) ; assert ! (link . file_type () . is_dir ()) ; assert ! (! link . metadata () . unwrap () . file_type () . is_symlink ()) ; assert ! (! link . metadata () . unwrap () . is_file ()) ; assert ! (link . metadata () . unwrap () . is_dir ()) ; let link_zzz = & ents [1] ; assert_eq ! (dir . join ("a-link") . join ("zzz") , link_zzz . path ()) ; assert ! (! link_zzz . path_is_symlink ()) ; }
};
}
