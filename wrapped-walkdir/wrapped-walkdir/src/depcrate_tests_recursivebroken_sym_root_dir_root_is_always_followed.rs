// Generated macro for broken_sym_root_dir_root_is_always_followed (function)
macro_rules! Depcrate_tests_recursivebroken_sym_root_dir_root_is_always_followed {
() => {
// Module: crate::tests::recursive
// Provides: {"broken_sym_root_dir_root_is_always_followed"}
// Dependencies: {}
# [test] fn broken_sym_root_dir_root_is_always_followed () { let dir = Dir :: tmp () ; dir . symlink_dir ("broken" , "a-link") ; for follow_symlinks in & [true , false] { let wd = WalkDir :: new (dir . join ("a-link")) . follow_links (* follow_symlinks) ; let r = dir . run_recursive (wd) ; assert ! (r . sorted_ents () . is_empty ()) ; assert_eq ! (r . errs () . len () , 1 , "broken symlink in roots cannot be traversed, they are always followed") ; } }
};
}
