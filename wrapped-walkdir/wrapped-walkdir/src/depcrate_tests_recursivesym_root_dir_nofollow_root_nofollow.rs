// Generated macro for sym_root_dir_nofollow_root_nofollow (function)
macro_rules! Depcrate_tests_recursivesym_root_dir_nofollow_root_nofollow {
() => {
// Module: crate::tests::recursive
// Provides: {"sym_root_dir_nofollow_root_nofollow"}
// Dependencies: {}
# [test] fn sym_root_dir_nofollow_root_nofollow () { let dir = Dir :: tmp () ; dir . mkdirp ("a") ; dir . symlink_dir ("a" , "a-link") ; dir . touch ("a/zzz") ; let wd = WalkDir :: new (dir . join ("a-link")) . follow_root_links (false) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let ents = r . sorted_ents () ; assert_eq ! (1 , ents . len ()) ; let link = & ents [0] ; assert_eq ! (dir . join ("a-link") , link . path ()) ; assert_eq ! (0 , link . depth ()) ; }
};
}
