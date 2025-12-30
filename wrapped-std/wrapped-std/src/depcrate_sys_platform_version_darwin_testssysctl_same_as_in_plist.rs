// Generated macro for sysctl_same_as_in_plist (function)
macro_rules! Depcrate_sys_platform_version_darwin_testssysctl_same_as_in_plist {
() => {
// Module: crate::sys::platform_version::darwin::tests
// Provides: {"sysctl_same_as_in_plist"}
// Dependencies: {}
# [test] fn sysctl_same_as_in_plist () { if let Some (version) = version_from_sysctl () { assert_eq ! (version , version_from_plist ()) ; } }
};
}
