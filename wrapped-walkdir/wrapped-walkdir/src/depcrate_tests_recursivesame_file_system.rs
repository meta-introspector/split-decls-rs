// Generated macro for same_file_system (function)
macro_rules! Depcrate_tests_recursivesame_file_system {
() => {
// Module: crate::tests::recursive
// Provides: {"same_file_system"}
// Dependencies: {}
# [cfg (target_os = "linux")] # [test] fn same_file_system () { use std :: path :: Path ; if ! Path :: new ("/sys") . is_dir () { return ; } let dir = Dir :: tmp () ; dir . touch ("a") ; dir . symlink_dir ("/sys" , "sys-link") ; let wd = WalkDir :: new (dir . path ()) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("a") , dir . join ("sys-link")] ; assert_eq ! (expected , r . sorted_paths ()) ; let wd = WalkDir :: new (dir . path ()) . same_file_system (true) . follow_links (true) ; let r = dir . run_recursive (wd) ; r . assert_no_errors () ; let expected = vec ! [dir . path () . to_path_buf () , dir . join ("a") , dir . join ("sys-link")] ; assert_eq ! (expected , r . sorted_paths ()) ; }
};
}
