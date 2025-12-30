// Generated macro for test (module)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_struct_local () { let vi = get_version_info ! () ; assert_eq ! (vi . major , 0) ; assert_eq ! (vi . minor , 4) ; assert_eq ! (vi . patch , 2) ; assert_eq ! (vi . crate_name , "rustc_tools_util") ; assert ! (vi . commit_hash . is_none ()) ; assert ! (vi . commit_date . is_none ()) ; assert ! (vi . host_compiler . is_none ()) ; } # [test] fn test_display_local () { let vi = get_version_info ! () ; assert_eq ! (vi . to_string () , "rustc_tools_util 0.4.2") ; } # [test] fn test_debug_local () { let vi = get_version_info ! () ; let s = format ! ("{vi:?}") ; assert_eq ! (s , "VersionInfo { crate_name: \"rustc_tools_util\", major: 0, minor: 4, patch: 2 }") ; } }
};
}
