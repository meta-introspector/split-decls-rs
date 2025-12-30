// Generated macro for broken_sym_root_dir_follow_and_root_nofollow (function)
macro_rules! Depcrate_tests_recursivebroken_sym_root_dir_follow_and_root_nofollow {
() => {
// Module: crate::tests::recursive
// Provides: {"broken_sym_root_dir_follow_and_root_nofollow"}
// Dependencies: {}
# [test] fn broken_sym_root_dir_follow_and_root_nofollow () { let dir = Dir :: tmp () ; dir . symlink_dir ("broken" , "a-link") ; let wd = WalkDir :: new (dir . join ("a-link")) . follow_links (true) . follow_root_links (false) ; let r = dir . run_recursive (wd) ; assert ! (r . sorted_ents () . is_empty ()) ; assert_eq ! (r . errs () . len () , 1 , "broken symlink cannot be traversed - they are followed if symlinks are followed") ; }
};
}
