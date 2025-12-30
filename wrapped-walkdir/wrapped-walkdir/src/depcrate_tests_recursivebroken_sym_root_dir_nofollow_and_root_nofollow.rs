// Generated macro for broken_sym_root_dir_nofollow_and_root_nofollow (function)
macro_rules! Depcrate_tests_recursivebroken_sym_root_dir_nofollow_and_root_nofollow {
() => {
// Module: crate::tests::recursive
// Provides: {"broken_sym_root_dir_nofollow_and_root_nofollow"}
// Dependencies: {}
# [test] fn broken_sym_root_dir_nofollow_and_root_nofollow () { let dir = Dir :: tmp () ; dir . symlink_dir ("broken" , "a-link") ; let wd = WalkDir :: new (dir . join ("a-link")) . follow_links (false) . follow_root_links (false) ; let r = dir . run_recursive (wd) ; let ents = r . sorted_ents () ; assert_eq ! (ents . len () , 1) ; let link = & ents [0] ; assert_eq ! (dir . join ("a-link") , link . path ()) ; assert ! (link . path_is_symlink ()) ; }
};
}
